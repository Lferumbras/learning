package domainservices

import "go-clean-simple/entities"

type OrderPricingService struct{}

func NewOrderPricingService() *OrderPricingService {
	return &OrderPricingService{}
}

func (s *OrderPricingService) CalculateTotal(items []entities.OrderItem) float64 {
	subtotal := 0.0

	for _, item := range items {
		subtotal += float64(item.Quantity) * item.UnitPrice
	}

	discount := s.calculateDiscount(subtotal)
	return subtotal - discount
}

func (s *OrderPricingService) calculateDiscount(subtotal float64) float64 {
	if subtotal >= 1000 {
		return subtotal * 0.10
	}

	if subtotal >= 500 {
		return subtotal * 0.05
	}

	return 0
}
