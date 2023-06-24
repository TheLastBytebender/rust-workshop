use std::cell::RefCell;
use std::collections::HashMap;

type OrderMap = RefCell<HashMap<String, Order>>;

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
	
	pub fn handle_mapping(&mut self, order_value: OrderStatus) -> &OrderMap {

		match order_value {
			OrderStatus::Active(order_position) => {
				match order_position {
					OrderPosition::BuySide(_) => {
						&self.buy_side_orders_active
					}

					OrderPosition::SellSide(_) => {
						&self.sell_side_orders_active
					}

					_ => todo!()
				}
			}

			OrderStatus::Pending(order_position) => {
				match order_position {
					OrderPosition::BuySide(_) => {
						&self.buy_side_orders_pending
					}

					OrderPosition::SellSide(_) => {
						&self.sell_side_orders_pending
					}

					_ => todo!()
				}
			}

			_ => todo!()
		}
	}
	
	pub fn add_order (&mut self, order: OrderStatus) {
		match order {
			OrderStatus::Active(order_position) => {
				match order_position {
					OrderPosition::BuySide(order) => {
						self.buy_side_orders_active
							.borrow_mut()
							.insert(order.id.clone(), order);
					}

					OrderPosition::SellSide(order) => {
						self.sell_side_orders_active
							.borrow_mut()
							.insert(order.id.clone(), order);
					}

					_ => todo!()
				}
			}

			OrderStatus::Pending(order_position) => {
				match order_position {
					OrderPosition::BuySide(order) => {
						self.buy_side_orders_pending
							.borrow_mut()
							.insert(order.id.clone(), order);
					}

					OrderPosition::SellSide(order) => {
						self.sell_side_orders_pending
							.borrow_mut()
							.insert(order.id.clone(), order);
					}

					_ => todo!()
				}
			}

			OrderStatus::Cancelled(_) => todo!()
		}
	}

	pub fn delete_order(&mut self, order_id:OrderStatus) {
		match order_id {
			OrderStatus::Active(order_position) => {
				match order_position {
					OrderPosition::BuySide(order_id) => {
						_ = self.buy_side_orders_active
							.borrow_mut()
							.remove(&order_id.id);
					}

					OrderPosition::SellSide(order_id) => {
						_ = self.sell_side_orders_active
							.borrow_mut()
							.remove(&order_id.id);
					}

					_ => todo!()
				}
			}

			OrderStatus::Pending(order_position) => {
				match order_position {
					OrderPosition::BuySide(order_id) => {
						_ = self.buy_side_orders_pending
							.borrow_mut()
							.remove(&order_id.id);
					}

					OrderPosition::SellSide(order_id) => {
						_ = self.sell_side_orders_pending
							.borrow_mut()
							.remove(&order_id.id);
					}

					_ => todo!()
				}
			},
			OrderStatus::Cancelled(_) => todo!()
		}
	}

	pub fn get_order(&self, order: OrderStatus) -> Order {
		match order {
			OrderStatus::Active(order_position) => {
				match order_position {
					OrderPosition::BuySideId(order_id) => {
						let value = self.buy_side_orders_active
							.borrow();

						let anw = value
							.get(&order_id)
							.expect("Failed to get active buy side order");

						anw.clone()
					}

					OrderPosition::SellSideId(order_id) => {
						let value = self.sell_side_orders_active
							.borrow();

						let anw = value
							.get(&order_id)
							.expect("Failed to get active sell side order");

						anw.clone()
					}

					_ => todo!()
				}
			}

			OrderStatus::Pending(order_position) => {
				match order_position {
					OrderPosition::BuySideId(order_id) => {
						let value = self.buy_side_orders_pending
							.borrow();

						let anw = value
							.get(&order_id)
							.expect("Failed to get pending buy side order");

						anw.clone()
					}

					OrderPosition::SellSideId(order_id) => {
						let value = self.sell_side_orders_pending
							.borrow();

						let anw = value
							.get(&order_id)
							.expect("Failed to get pending sell side order");

						anw.clone()
					}

					_ => todo!()
				}
			}

			_ => todo!()
		}
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_oms() {
    	let oms = Oms::new();

    	let oms_dummy = Oms {
    		sell_side_orders_active: RefCell::new(HashMap::new()),
			sell_side_orders_pending: RefCell::new(HashMap::new()),
			buy_side_orders_active: RefCell::new(HashMap::new()),
			buy_side_orders_pending: RefCell::new(HashMap::new())
    	};

    	assert_eq!(oms_dummy, oms)
    }

    #[test]
    fn test_add_order_oms() {
    	let mut oms = Oms::new();

    	let buy_order_active = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("1234"),
    				price: 12.5,
    				qty: 2.5,
    				position_idx: 1,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	let sell_order_active = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("12345"),
    				price: 12.5,
    				qty: 2.5,
    				position_idx: 2,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	oms.add_order(buy_order_active);
    	oms.add_order(sell_order_active);

    	assert_eq!(oms.buy_side_orders_active.borrow().len(), 1);
    	assert_eq!(oms.sell_side_orders_active.borrow().len(), 1);
    }

    #[test]
    fn test_delete_order_oms() {
    	let mut oms = Oms::new();

    	let buy_order_active = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("1234"),
    				price: 12.5,
    				qty: 2.5,
    				position_idx: 1,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	let sell_order_active = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("12345"),
    				price: 12.5,
    				qty: 2.5,
    				position_idx: 2,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	oms.add_order(buy_order_active.clone());
    	oms.add_order(sell_order_active.clone());

    	assert_eq!(oms.buy_side_orders_active.borrow().len(), 1);
    	assert_eq!(oms.sell_side_orders_active.borrow().len(), 1);

    	oms.delete_order(buy_order_active);
    	oms.delete_order(sell_order_active);

    	assert_eq!(oms.buy_side_orders_active.borrow().len(), 0);
    	assert_eq!(oms.sell_side_orders_active.borrow().len(), 0);

    }

    #[test]
    fn test_get_order_oms() {
    	let mut oms = Oms::new();

    	let buy_order_active = 
			Order {
				id: String::from("1234"),
				price: 12.5,
				qty: 2.5,
				position_idx: 1,
				created_time: 1_000_000,
				updated_time: 1_000_100
			};

    	let sell_order_pending = 
			Order {
				id: String::from("12345"),
				price: 12.5,
				qty: 2.5,
				position_idx: 2,
				created_time: 1_000_000,
				updated_time: 1_000_100
			};

		let sell_order_active = 
			Order {
				id: String::from("1234554"),
				price: 12.5,
				qty: 2.5,
				position_idx: 2,
				created_time: 1_000_000,
				updated_time: 1_000_100
			};

		// I dont like all these clones
		// Will be fixing
    	oms.add_order(OrderStatus::Active(OrderPosition::BuySide(buy_order_active.clone())));
    	oms.add_order(OrderStatus::Pending(OrderPosition::SellSide(sell_order_pending.clone())));
    	oms.add_order(OrderStatus::Active(OrderPosition::SellSide(sell_order_active.clone())));

    	let test1 = oms.get_order(OrderStatus::Active(OrderPosition::BuySideId(buy_order_active.id.clone())));
    	let test2 = oms.get_order(OrderStatus::Pending(OrderPosition::SellSideId(sell_order_pending.id.clone())));
    	let test3 = oms.get_order(OrderStatus::Active(OrderPosition::SellSideId(sell_order_active.id.clone())));


    	assert_eq!(test1.clone() , buy_order_active);
    	assert_eq!(test2.clone(), sell_order_pending);
    	assert_eq!(test3.clone(), sell_order_active);
    }

    #[test]
    fn test_get_inventory_delta_oms() {

    	let mut oms = Oms::new();

    	let sell_order_active = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("1232432445"),
    				price: 12.5,
    				qty: 3.0,
    				position_idx: 2,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	let sell_order_active_2 = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("123423445"),
    				price: 12.5,
    				qty: 10.0,
    				position_idx: 2,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	let buy_order_active = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("123234244"),
    				price: 12.5,
    				qty: 5.0,
    				position_idx: 1,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	let buy_order_active_2 = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("12342342345"),
    				price: 12.5,
    				qty: 10.0,
    				position_idx: 1,
    				created_time: 1_000_000,
    				updated_time: 1_000_100
    			}
    		)
    	);

    	oms.add_order(sell_order_active);
    	oms.add_order(sell_order_active_2);
    	oms.add_order(buy_order_active);
    	oms.add_order(buy_order_active_2);

    	let anw = 15.0 - 13.0;
    	let result = oms.get_inventory_delta();

    	assert_eq!(anw, result);

    }
}