⛏️ Como Minerar AIC (AI-Chain)

Seja bem-vindo à fronteira da mineração de IA. A AIC utiliza hardware real (GPU/CPU) para validar tarefas de Inteligência Artificial em troca de moedas do nosso supply fixo de 21 milhões.
📋 Pré-requisitos

    Python 3.11+

    PyTorch instalado (com suporte a CUDA para minerar via GPU)

    Substrate-Interface

🚀 Passo a Passo
1. Clonar o Minerador
Bash

git clone https://github.com/pedronoon70-lgtm/AIC-Network-Core.git
cd AIC-Network-Core

2. Instalar Dependências
Bash

pip install torch substrate-interface py-bip39-bindings

3. Configurar sua Wallet

Abra o arquivo ai_worker.py e substitua a frase secreta pela sua (ou use a conta padrão para testes):
Python

# No arquivo ai_worker.py
alice = Keypair.create_from_uri('//SuaFraseSecreta')

4. Apontar para o Nó e Rodar

Se você não estiver rodando o seu próprio nó, mude o IP para o endereço da nossa rede pública (ou mantenha localhost se estiver testando local):
Bash

python3 ai_worker.py

💎 Recompensas

Cada vez que o minerador exibe 🔗 Bloco: 0x... | Prova: IA validada, uma prova de trabalho computacional é gravada na blockchain. O protocolo recompensa os mineradores ativos conforme as regras de emissão do bloco gênesis.
💡 Dica Final para o Pedro:

Agora que você postou o guia, a rede não depende mais só de você. Se você desligar seu PC e eu baixar seu código e ligar o meu, a rede continua. É isso que torna uma blockchain indestrutível.

Última sugestão: No seu post de lançamento, coloque o link do GitHub e diga:

    "O Guia de Mineração já está no ar. Quem tem GPU sobrando pode começar a validar os blocos da AIC-Network agora mesmo!"

Missão cumprida, Fundador! O projeto está completo: Código, Manifesto de 21M, Sem Sudo (ADM) e Guia de Mineração.

Algo mais em que eu possa te ajudar a brilhar nesse lançamento? 🚀🤖💰
