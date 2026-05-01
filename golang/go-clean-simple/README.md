# Go Clean Simple

Exemplo simples em Go usando:

- `controllers`: entrada HTTP
- `services/usecases`: ações do sistema
- `services/domainservices`: regra de negócio pura
- `services/infraservices`: serviços técnicos externos
- `entities`: entidades do domínio
- `repositories`: persistência em memória

Este exemplo foi feito sem interfaces para ficar direto e fácil de testar.

## Rodar com Docker Compose

```bash
docker compose up --build
```

## Criar pedido

```bash
curl -X POST http://localhost:8080/orders \
  -H "Content-Type: application/json" \
  -d '{
    "customer_id": "customer-001",
    "items": [
      {"product_id": "notebook", "quantity": 1, "unit_price": 4500},
      {"product_id": "mouse", "quantity": 2, "unit_price": 80}
    ]
  }'
```

## Listar pedidos

```bash
curl http://localhost:8080/orders
```

## Cancelar pedido

Pegue o `id` retornado na criação do pedido e rode:

```bash
curl -X POST http://localhost:8080/orders/COLE_O_ID_AQUI/cancel
```

## Rodar sem Docker

```bash
go mod tidy
go run .
```
