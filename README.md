# 1ª Rust API 👨‍💻
**DEV-SAM**: S.Fernandes.(O último mestre do Python)

## 📍 Primeiro Passo
    curl --request GET \
    --url https://api-send-emails.aplicacao-tech.com.br/routes/token

## 📍 Segundo Passo
    curl --request POST \
      --url https://api-send-emails.aplicacao-tech.com.br/routes/send-emails/SEU_TOKEN_RETORNADO_DO_PASSO_ACIMA \
      --header 'Content-Type: application/json' \
      --data '{
      "emails": [
        {
          "email": "email-example@gmail.com",
          "title": "olá, mundo",
          "body": "Enviando email1"
        },
        {
          "email": "email-example@gmail.com",
          "title": "olá, mundo",
          "body": "Enviando email"
        }
      ]
    }
    '

## Resultado Final 😝
   ### Por token, a API pode enviar até 30 Emails diários,
   ### no outro dia é resetado o limite e assim sucessivamente...
   ### A  API enviar até 300 Emails por dia!
