# RustDesk customizado — senha fixa de acesso não assistido

Esta build personaliza o RustDesk para que **toda máquina onde o cliente instalar
o programa aceite uma senha permanente fixa** para acesso não assistido. Assim, o
operador de suporte só precisa do **ID** do cliente e da **senha fixa** para
conectar — sem depender da senha temporária que muda a cada sessão.

## O que muda em relação ao RustDesk original

- Nada mais muda: o programa continua funcionando normalmente (ID, senha
  temporária, tudo igual).
- A **única diferença** é que a senha **permanente** é sempre definida para um
  valor fixo assim que o serviço do RustDesk sobe na máquina do cliente.
- A senha permanente padrão desta build é: **`TESTE123`**.

## Como funciona (implementação)

- `src/custom_password.rs`
  - Constante `CUSTOM_FIXED_PASSWORD` — o valor fixo (`TESTE123` por padrão).
  - Função `enforce_custom_fixed_password()` — chama a própria rotina de produção
    `Config::set_permanent_password(...)`, então o hash/salt são gerados
    exatamente como quando o usuário digita a senha na interface. É idempotente:
    depois da primeira gravação o salt é reaproveitado e as chamadas seguintes
    viram no-op (não regravam o config).
- `src/server.rs`
  - A função é chamada no início de `start_server(is_server = true)`, ou seja,
    **no processo servidor** que valida as conexões recebidas. Isso roda na
    instalação, a cada boot e sempre que o serviço reinicia — mantendo a senha
    fixa mesmo que alguém a altere.

Como usamos a senha **permanente local** (e não o mecanismo de "preset password"
de edições OEM), o cliente **não** vê o aviso vermelho de "senha predefinida": a
máquina parece um RustDesk normal que apenas já tem uma senha permanente
configurada.

## Trocar a senha fixa (opcional)

O valor pode ser alterado em tempo de compilação, sem editar código, exportando a
variável de ambiente antes do build:

```bash
export RUSTDESK_FIXED_PASSWORD='SuaSenhaForte'
```

Se a variável não existir, o padrão `TESTE123` é usado.

## Como gerar o instalador `.exe` para Windows

O `.exe` do Windows **precisa ser compilado num ambiente Windows** (Rust + Flutter
+ dependências nativas via vcpkg). Não é possível compilá-lo em Linux. Há dois
caminhos:

### Opção A — GitHub Actions (recomendado, não precisa de máquina Windows)

O repositório já contém o workflow de build do Windows.

1. Garanta que **Actions esteja habilitado** no seu fork
   (aba *Actions* → *I understand my workflows, go ahead and enable them*).
2. Dispare o build de uma destas formas:
   - **Nightly manual:** aba *Actions* → workflow **"Flutter Nightly Build"** →
     *Run workflow* (usa `workflow_dispatch`).
   - **Por tag de versão:** crie e envie uma tag no formato `1.4.5`
     (ou `v1.4.5`). O workflow **"Flutter Tag Build"** roda automaticamente.
3. Quando o build terminar, baixe o artefato/asset
   `rustdesk-<versão>-x86_64.exe` — esse é o instalador autoextraível que você
   envia aos clientes.

> Observação: os passos de **assinatura de código** usam segredos que só existem
> no repositório oficial. Sem esses segredos, o build gera o `.exe` **sem
> assinatura digital** (funciona, mas o Windows SmartScreen pode exibir aviso).
> Para produção, configure seu próprio certificado nos segredos do repositório.

### Opção B — build local numa máquina Windows

Pré-requisitos do RustDesk (Rust, LLVM, Flutter, vcpkg — ver `README.md`).
Depois:

```bash
# opcional: trocar a senha fixa
set RUSTDESK_FIXED_PASSWORD=SuaSenhaForte

python build.py --portable --flutter
```

Isso gera o `rustdesk.exe`; o empacotador autoextraível fica em
`libs/portable/` (ver os passos "Build self-extracted executable" no workflow
`.github/workflows/flutter-build.yml`).

## Como o operador conecta

1. O cliente instala o `.exe` e informa apenas o **ID** que aparece no RustDesk
   dele.
2. No seu RustDesk padrão, digite o **ID** do cliente e a senha **`TESTE123`**.
3. Pronto — acesso não assistido à máquina do cliente.

## ⚠️ Aviso de segurança (importante)

Uma senha **única, fixa e fraca**, igual em **todas** as máquinas, é um risco
sério: qualquer pessoa que descobrir a senha e o ID de um cliente ganha controle
total do computador dele. Recomendações fortes:

- Use uma senha **longa e aleatória** (via `RUSTDESK_FIXED_PASSWORD`), não
  `TESTE123`, em produção.
- De preferência, use um **servidor RustDesk próprio (self-hosted)** e o
  controle de acesso por conta/lista de permissão, em vez de depender só de uma
  senha compartilhada.
- Considere senhas **por cliente** em vez de uma única para todos.

`TESTE123` é adequado apenas para testes.
