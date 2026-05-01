package controllers

import (
	"encoding/json"
	"net/http"
	"strings"

	"go-clean-simple/entities"
	"go-clean-simple/repositories"
	"go-clean-simple/services/usecases"
)

type OrderController struct {
	createOrderUseCase *usecases.CreateOrderUseCase
	cancelOrderUseCase *usecases.CancelOrderUseCase
	orderRepository    *repositories.OrderRepository
}

func NewOrderController(
	createOrderUseCase *usecases.CreateOrderUseCase,
	cancelOrderUseCase *usecases.CancelOrderUseCase,
	orderRepository *repositories.OrderRepository,
) *OrderController {
	return &OrderController{
		createOrderUseCase: createOrderUseCase,
		cancelOrderUseCase: cancelOrderUseCase,
		orderRepository:    orderRepository,
	}
}

type CreateOrderRequest struct {
	CustomerID string               `json:"customer_id"`
	Items      []entities.OrderItem `json:"items"`
}

func (c *OrderController) Create(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		writeError(w, http.StatusMethodNotAllowed, "method not allowed")
		return
	}

	var request CreateOrderRequest
	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		writeError(w, http.StatusBadRequest, "invalid request body")
		return
	}

	order, err := c.createOrderUseCase.Execute(usecases.CreateOrderInput{
		CustomerID: request.CustomerID,
		Items:      request.Items,
	})
	if err != nil {
		writeError(w, http.StatusBadRequest, err.Error())
		return
	}

	writeJSON(w, http.StatusCreated, order)
}

func (c *OrderController) List(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		writeError(w, http.StatusMethodNotAllowed, "method not allowed")
		return
	}

	orders, err := c.orderRepository.FindAll()
	if err != nil {
		writeError(w, http.StatusInternalServerError, err.Error())
		return
	}

	writeJSON(w, http.StatusOK, orders)
}

func (c *OrderController) Cancel(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		writeError(w, http.StatusMethodNotAllowed, "method not allowed")
		return
	}

	// Esperado: /orders/{id}/cancel
	path := strings.Trim(r.URL.Path, "/")
	parts := strings.Split(path, "/")
	if len(parts) != 3 || parts[0] != "orders" || parts[2] != "cancel" {
		writeError(w, http.StatusNotFound, "route not found")
		return
	}

	orderID := parts[1]
	order, err := c.cancelOrderUseCase.Execute(orderID)
	if err != nil {
		writeError(w, http.StatusBadRequest, err.Error())
		return
	}

	writeJSON(w, http.StatusOK, order)
}

func writeJSON(w http.ResponseWriter, statusCode int, data any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(statusCode)
	_ = json.NewEncoder(w).Encode(data)
}

func writeError(w http.ResponseWriter, statusCode int, message string) {
	writeJSON(w, statusCode, map[string]string{"error": message})
}
