package repositories

import (
	"sync"

	"go-clean-simple/entities"
)

type OrderRepository struct {
	mu     sync.RWMutex
	orders map[string]*entities.Order
}

func NewOrderRepository() *OrderRepository {
	return &OrderRepository{
		orders: make(map[string]*entities.Order),
	}
}

func (r *OrderRepository) Save(order *entities.Order) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	r.orders[order.ID] = order
	return nil
}

func (r *OrderRepository) FindByID(id string) (*entities.Order, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	order, ok := r.orders[id]
	if !ok {
		return nil, nil
	}

	return order, nil
}

func (r *OrderRepository) FindAll() ([]*entities.Order, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	orders := make([]*entities.Order, 0, len(r.orders))
	for _, order := range r.orders {
		orders = append(orders, order)
	}

	return orders, nil
}
