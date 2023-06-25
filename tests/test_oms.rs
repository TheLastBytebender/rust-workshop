use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;

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

/* 
TESTS ARE HERE
*/

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
    	let mut rng = rand::thread_rng();

    	let buy_order_active = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("1234"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
    			}
    		)
    	);

    	let sell_order_active = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("123465"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
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
    	let mut rng = rand::thread_rng();

    	let buy_order_active = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("1234"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
    			}
    		)
    	);

    	let sell_order_active = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("12345"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
    			}
    		)
    	);

    	oms.add_order(buy_order_active.clone());
    	oms.add_order(sell_order_active.clone());

    	assert_eq!(oms.buy_side_orders_active.borrow().len(), 1);
    	assert_eq!(oms.sell_side_orders_active.borrow().len(), 1);

    	oms.delete_order(OrderStatus::Active(OrderPosition::BuySideId("1234".to_string())));
    	oms.delete_order(OrderStatus::Active(OrderPosition::SellSideId("12345".to_string())));

    	assert_eq!(oms.buy_side_orders_active.borrow().len(), 0);
    	assert_eq!(oms.sell_side_orders_active.borrow().len(), 0);

    }

    #[test]
    fn test_get_order_oms() {
    	let mut oms = Oms::new();
    	let mut rng = rand::thread_rng();

    	let buy_order_active = 
			Order {
				id: String::from("1234"),
				price: rng.gen::<f64>(),
				qty: rng.gen::<f64>(),
				position_idx: rng.gen::<u8>(),
				created_time: rng.gen::<i32>(),
				updated_time: rng.gen::<i32>()
			};

    	let sell_order_pending = 
			Order {
				id: String::from("12345"),
				price: rng.gen::<f64>(),
				qty: rng.gen::<f64>(),
				position_idx: rng.gen::<u8>(),
				created_time: rng.gen::<i32>(),
				updated_time: rng.gen::<i32>()
			};

		let sell_order_active = 
			Order {
				id: String::from("123456"),
				price: rng.gen::<f64>(),
				qty: rng.gen::<f64>(),
				position_idx: rng.gen::<u8>(),
				created_time: rng.gen::<i32>(),
				updated_time: rng.gen::<i32>()
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
    	let mut rng = rand::thread_rng();

    	let sell_order_active = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("1234"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
    			}
    		)
    	);

    	let sell_order_active_2 = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("12345"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
    			}
    		)
    	);

    	let buy_order_active = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("123456"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
    			}
    		)
    	);

    	let buy_order_active_2 = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("1234567"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
    			}
    		)
    	);

    	let OrderStatus::Active(OrderPosition::BuySide(b_1)) = &buy_order_active else { todo!() }; 
    	let OrderStatus::Active(OrderPosition::BuySide(b_2)) = &buy_order_active_2 else { todo!() };

    	let OrderStatus::Active(OrderPosition::SellSide(a_1)) = &sell_order_active else { todo!() };
    	let OrderStatus::Active(OrderPosition::SellSide(a_2)) = &sell_order_active_2 else { todo!() };

    	let delta = ((b_1.qty + b_2.qty) - (a_1.qty + a_2.qty)) as f64;

    	oms.add_order(sell_order_active);
    	oms.add_order(sell_order_active_2);
    	oms.add_order(buy_order_active);
    	oms.add_order(buy_order_active_2);

    	let result = oms.get_inventory_delta();

    	assert_eq!(delta, result);
    }

    #[test]
    fn test_get_size_to_target() {

    	let mut oms = Oms::new();
    	let mut rng = rand::thread_rng();

    	let buy_order_active = OrderStatus::Active (
    		OrderPosition::BuySide (
    			Order {
    				id: String::from("1234"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
    			}
    		)
    	);

    	let sell_order_active = OrderStatus::Active (
    		OrderPosition::SellSide (
    			Order {
    				id: String::from("12345"),
    				price: rng.gen::<f64>(),
    				qty: rng.gen::<f64>(),
    				position_idx: rng.gen::<u8>(),
    				created_time: rng.gen::<i32>(),
    				updated_time: rng.gen::<i32>()
    			}
    		)
    	);

    	let OrderStatus::Active(OrderPosition::BuySide(b_1)) = &buy_order_active else { todo!() }; 
    	let OrderStatus::Active(OrderPosition::SellSide(a_1)) = &sell_order_active else { todo!() };

    	let target_delta = 0.0;
    	let anw = ((b_1.qty - a_1.qty) - target_delta as f64).abs();

    	oms.add_order(buy_order_active);
    	oms.add_order(sell_order_active);

    	let result = oms.get_size_to_target(target_delta);

    	assert_eq!(anw, result);
    }
}