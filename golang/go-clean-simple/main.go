package main

import (
	"fmt"
	"log"
	"net/http"

	"go-clean-simple/controllers"
	"go-clean-simple/repositories"
	"go-clean-simple/services/domainservices"
	"go-clean-simple/services/infraservices"
	"go-clean-simple/services/usecases"
)

func main() {
	// Dependências concretas. Sem interface por enquanto.
	orderRepository := repositories.NewOrderRepository()
	orderPricingService := domainservices.NewOrderPricingService()
	emailService := infraservices.NewEmailService()

	createOrderUseCase := usecases.NewCreateOrderUseCase(
		orderRepository,
		orderPricingService,
		emailService,
	)

	cancelOrderUseCase := usecases.NewCancelOrderUseCase(orderRepository)

	orderController := controllers.NewOrderController(
		createOrderUseCase,
		cancelOrderUseCase,
		orderRepository,
	)

	http.HandleFunc("/orders", func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			orderController.Create(w, r)
		case http.MethodGet:
			orderController.List(w, r)
		default:
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		}
	})

	http.HandleFunc("/orders/", orderController.Cancel)

	fmt.Println("Server running on http://localhost:8080")
	fmt.Println("POST   /orders")
	fmt.Println("GET    /orders")
	fmt.Println("POST   /orders/{id}/cancel")

	log.Fatal(http.ListenAndServe(":8080", nil))
}
