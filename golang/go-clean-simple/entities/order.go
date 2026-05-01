package entities

import (
	"crypto/rand"
	"encoding/hex"
	"errors"
)

type OrderStatus string

const (
	OrderStatusCreated  OrderStatus = "CREATED"
	OrderStatusPaid     OrderStatus = "PAID"
	OrderStatusCanceled OrderStatus = "CANCELED"
)

type OrderItem struct {
	ProductID string  `json:"product_id"`
	Quantity  int     `json:"quantity"`
	UnitPrice float64 `json:"unit_price"`
}

type Order struct {
	ID         string      `json:"id"`
	CustomerID string      `json:"customer_id"`
	Items      []OrderItem `json:"items"`
	Total      float64     `json:"total"`
	Status     OrderStatus `json:"status"`
}

func NewOrder(customerID string, items []OrderItem, total float64) (*Order, error) {
	if customerID == "" {
		return nil, errors.New("customer_id is required")
	}

	if len(items) == 0 {
		return nil, errors.New("order must have at least one item")
	}

	for _, item := range items {
		if item.ProductID == "" {
			return nil, errors.New("product_id is required")
		}

		if item.Quantity <= 0 {
			return nil, errors.New("item quantity must be greater than zero")
		}

		if item.UnitPrice <= 0 {
			return nil, errors.New("item unit_price must be greater than zero")
		}
	}

	if total <= 0 {
		return nil, errors.New("order total must be greater than zero")
	}

	return &Order{
		ID:         newID(),
		CustomerID: customerID,
		Items:      items,
		Total:      total,
		Status:     OrderStatusCreated,
	}, nil
}

func (o *Order) Cancel() error {
	if o.Status == OrderStatusPaid {
		return errors.New("cannot cancel a paid order")
	}

	if o.Status == OrderStatusCanceled {
		return errors.New("order is already canceled")
	}

	o.Status = OrderStatusCanceled
	return nil
}

func newID() string {
	b := make([]byte, 16)
	if _, err := rand.Read(b); err != nil {
		return "fallback-id"
	}

	return hex.EncodeToString(b)
}
