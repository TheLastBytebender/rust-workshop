
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Order {
	id: String,
	price: f64,
	qty: f64,
	position_idx: u8,
	created_time: i32,
	updated_time: i32,
}

#[derive(Clone)]
enum OrderPosition {
	BuySide(Order),
	SellSide(Order),
	BuySideId(String),
	SellSideId(String)
}

#[derive(Clone)]
enum OrderStatus {
	Pending(OrderPosition),
	Active(OrderPosition),
	Cancelled(OrderPosition)
}

#[derive(Debug, PartialEq)]
struct Oms {
	sell_side_orders_active: HashMap<String, Order>,
	sell_side_orders_pending: HashMap<String, Order>,
	buy_side_orders_active: HashMap<String, Order>,
	buy_side_orders_pending: HashMap<String, Order>
}

impl Oms {

	pub fn new() -> Oms {
		Oms { 
			sell_side_orders_active: HashMap::new(),
			sell_side_orders_pending: HashMap::new(),
			buy_side_orders_active: HashMap::new(),
			buy_side_orders_pending: HashMap::new()
		}
	}

	pub fn add_order (&mut self, order: OrderStatus) {
		match order {
			OrderStatus::Active(order_position) => {
				match order_position {
					OrderPosition::BuySide(order) => {
						self.buy_side_orders_active
							.insert(order.id.clone(), order);
					}

					OrderPosition::SellSide(order) => {
						self.sell_side_orders_active
							.insert(order.id.clone(), order);
					}

					_ => todo!()
				}
			}

			OrderStatus::Pending(order_position) => {
				match order_position {
					OrderPosition::BuySide(order) => {
						self.buy_side_orders_pending
							.insert(order.id.clone(), order);
					}

					OrderPosition::SellSide(order) => {
						self.sell_side_orders_pending
							.insert(order.id.clone(), order);
					}

					_ => todo!()
				}
			}

			OrderStatus::Cancelled(_) => todo!()
		}
	}

	pub fn get_order(&self, order: OrderStatus) -> &Order {
		match order {
			OrderStatus::Active(order_position) => {
				match order_position {
					OrderPosition::BuySideId(order_id) => {
						let value = self.buy_side_orders_active
							.get(&order_id)
							.expect("Failed to get active buy side order");

						value
					}

					OrderPosition::SellSideId(order_id) => {
						let value = self.sell_side_orders_active
							.get(&order_id)
							.expect("Failed to get active sell side order");

						value
					}

					_ => todo!()
				}
			}

			OrderStatus::Pending(order_position) => {
				match order_position {
					OrderPosition::BuySideId(order_id) => {
						let value = self.buy_side_orders_pending
							.get(&order_id)
							.expect("Failed to get pending buy side order");

						value
					}

					OrderPosition::SellSideId(order_id) => {
						let value = self.sell_side_orders_pending
							.get(&order_id)
							.expect("Failed to get pending sell side order");

						value
					}

					_ => todo!()
				}
			}

			_ => todo!()
		}
	}
}
