# 1ª Rust API 👨‍💻
**DEV-SAM**: S.Fernandes.(O último mestre do Python)

## 📍 Primeiro Passo
    curl --request GET \
    --url https://api-send-emails.aplicacao-tech.com.br/routes/token

## 📍 Segundo Passo
    curl --request POST \
      --url https://api-send-emails.aplicacao-tech.com.br/routes/send-emails/SEU_TOKEN_RETORNADO DO PASSO ACIMA \
      --header 'Content-Type: application/json' \
      --data '{
      "emails": [
        {
          "email": "samueldiablo73@gmail.com",
          "title": "olá, mundo",
          "body": "Enviando email1"
        },
        {
          "email": "samuelfernandes2196@gmail.com",
          "title": "olá, mundo",
          "body": "Enviando email"
        }
      ]
    }
    '

## Resultado Final 😝
    Por token pode enviar até 30 Emails diários
    A  API envia 300 Emails por dia!
