package infraservices

import "fmt"

type EmailService struct{}

func NewEmailService() *EmailService {
	return &EmailService{}
}

func (s *EmailService) SendOrderCreatedEmail(customerID string, orderID string) error {
	fmt.Printf("[EMAIL] Pedido %s criado para o cliente %s\n", orderID, customerID)
	return nil
}
