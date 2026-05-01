package usecases

import (
	"errors"

	"go-clean-simple/entities"
	"go-clean-simple/repositories"
)

type CancelOrderUseCase struct {
	orderRepository *repositories.OrderRepository
}

func NewCancelOrderUseCase(orderRepository *repositories.OrderRepository) *CancelOrderUseCase {
	return &CancelOrderUseCase{orderRepository: orderRepository}
}

func (u *CancelOrderUseCase) Execute(orderID string) (*entities.Order, error) {
	order, err := u.orderRepository.FindByID(orderID)
	if err != nil {
		return nil, err
	}

	if order == nil {
		return nil, errors.New("order not found")
	}

	if err := order.Cancel(); err != nil {
		return nil, err
	}

	if err := u.orderRepository.Save(order); err != nil {
		return nil, err
	}

	return order, nil
}
