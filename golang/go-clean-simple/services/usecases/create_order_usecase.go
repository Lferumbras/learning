package usecases

import (
	"go-clean-simple/entities"
	"go-clean-simple/repositories"
	"go-clean-simple/services/domainservices"
	"go-clean-simple/services/infraservices"
)

type CreateOrderInput struct {
	CustomerID string               `json:"customer_id"`
	Items      []entities.OrderItem `json:"items"`
}

type CreateOrderUseCase struct {
	orderRepository *repositories.OrderRepository
	orderPricing    *domainservices.OrderPricingService
	emailService    *infraservices.EmailService
}

func NewCreateOrderUseCase(
	orderRepository *repositories.OrderRepository,
	orderPricing *domainservices.OrderPricingService,
	emailService *infraservices.EmailService,
) *CreateOrderUseCase {
	return &CreateOrderUseCase{
		orderRepository: orderRepository,
		orderPricing:    orderPricing,
		emailService:    emailService,
	}
}

func (u *CreateOrderUseCase) Execute(input CreateOrderInput) (*entities.Order, error) {
	total := u.orderPricing.CalculateTotal(input.Items)

	order, err := entities.NewOrder(input.CustomerID, input.Items, total)
	if err != nil {
		return nil, err
	}

	if err := u.orderRepository.Save(order); err != nil {
		return nil, err
	}

	if err := u.emailService.SendOrderCreatedEmail(order.CustomerID, order.ID); err != nil {
		return nil, err
	}

	return order, nil
}
