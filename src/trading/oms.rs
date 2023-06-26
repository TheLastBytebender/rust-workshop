use std::cell::RefCell;
use std::collections::HashMap;
/*

This module aims to create a localized order management system for 
tracking and analyzing trading inventory

*/

pub mod trading {
	type OrderMap = RefCell<HashMap<String, Order>>;

	enum AltReturn<'a> {
		SomeOrder((&'a OrderMap, Order)),
		SomeOrderId((&'a OrderMap, String))
	}

	#[derive(Clone)]
	enum OrderPosition {
		BuySide(Order), // Use when you need the order returned
		SellSide(Order), // Use when you need the order returned
		BuySideId(String), // Use when you need the order_id returned
		SellSideId(String) // Use when you need the order_id returned
	}

	#[derive(Clone)]
	enum OrderStatus {
		Pending(OrderPosition),
		Active(OrderPosition)
	}

	#[derive(Debug, PartialEq, Clone)]
	struct Order {
		id: String,
		price: f64,
		qty: f64,
		position_idx: u8,
		created_time: i32,
		updated_time: i32,
	}

	#[derive(Debug, PartialEq)]
	struct Oms {
		sell_side_orders_active: OrderMap,
		sell_side_orders_pending: OrderMap,
		buy_side_orders_active: OrderMap,
		buy_side_orders_pending: OrderMap
	}

	impl Oms {

		pub fn new() -> Oms {
			Oms { 
				sell_side_orders_active: RefCell::new(HashMap::new()),
				sell_side_orders_pending: RefCell::new(HashMap::new()),
				buy_side_orders_active: RefCell::new(HashMap::new()),
				buy_side_orders_pending: RefCell::new(HashMap::new())
			}
		}
		
		pub fn handle_mapping(&mut self, order_value: OrderStatus) -> AltReturn {

			match order_value {

				OrderStatus::Active(order_position) => {

					match order_position {
						OrderPosition::BuySide(order) => {
							AltReturn::SomeOrder((&self.buy_side_orders_active, order))
						}

						OrderPosition::SellSide(order) => {
							AltReturn::SomeOrder((&self.sell_side_orders_active, order))
						}

						OrderPosition::BuySideId(order_id) => {
							AltReturn::SomeOrderId((&self.buy_side_orders_active, order_id))
						}

						OrderPosition::SellSideId(order_id) => {
							AltReturn::SomeOrderId((&self.sell_side_orders_active, order_id))
						}
					}
				}

				OrderStatus::Pending(order_position) => {

					match order_position {
						OrderPosition::BuySide(order) => {
							AltReturn::SomeOrder((&self.buy_side_orders_pending, order))
						}

						OrderPosition::SellSide(order) => {
							AltReturn::SomeOrder((&self.sell_side_orders_pending, order))
						}

						OrderPosition::BuySideId(order_id) => {
							AltReturn::SomeOrderId((&self.buy_side_orders_pending, order_id))
						}

						OrderPosition::SellSideId(order_id) => {
							AltReturn::SomeOrderId((&self.sell_side_orders_pending, order_id))
						}
					}
				}
			}
		}
		
		pub fn add_order (&mut self, order_value: OrderStatus) {

			let AltReturn::SomeOrder((map, order)) = self.handle_mapping(order_value) else { panic!() };

			map
				.borrow_mut()
				.insert(order.id.clone(), order);
		}

		pub fn delete_order(&mut self, order_value: OrderStatus) {

			let AltReturn::SomeOrderId((map, order_id)) = self.handle_mapping(order_value) else { panic!() };

			map 
				.borrow_mut()
				.remove(&order_id);
		}

		pub fn get_order(&mut self, order_value: OrderStatus) -> Order {

			let AltReturn::SomeOrderId((map, order_id)) = self.handle_mapping(order_value) else { panic!() };

			let binding_map = map.borrow();

			let anw = binding_map
				.get(&order_id)
				.expect("Failed to get order");

			anw.clone()

		}
		// Will return current inventory delta
		pub fn get_inventory_delta(&self) -> f64 {

			let bid_delta: f64 = self.buy_side_orders_active
				.borrow()
				.values()
				.map(|order| order.qty)
				.sum();

			println!("{:?}", bid_delta);

			let ask_delta: f64 = self.sell_side_orders_active
				.borrow()
				.values()
				.map(|order| order.qty)
				.sum();

			println!("{:?}", ask_delta);

			let value = bid_delta - ask_delta;

			value
		}

		pub fn get_size_to_target(&self, target_delta: f64) -> f64 {
			
			let current_delta = self
				.get_inventory_delta();

			let anw = (current_delta - target_delta).abs();

			anw
		}
	}
}