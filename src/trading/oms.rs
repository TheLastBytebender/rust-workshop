
struct Order {
	id: str,
	price: f64,
	qty: f64,
	order_status: str,
	position_idx: u8,
	created_time: i32,
	updated_time: i32,
}

enum OrderPosition {
	BuySide(Order),
	SellSide(Order)
}

enum OrderStatus {
	Pending(OrderPosition),
	Active(OrderPosition),
	Cancelled(OrderPosition)
}

struct Oms {
	sell_side_orders_active: HashMap<str, Order>,
	sell_side_orders_pending: HashMap<str, Order>,
	buy_side_orders_active: HashMap<str, Order>,
	buy_side_orders_pending: HashMap<str, Order>
}

impl Order {

	pub fn add_order(&mut self, order: OrderStatus) {
		
		match order {
			OrderStatus::Active(order_position) => {
				match order_position {
					OrderPosition::BuySide(order) => {
						self.buy_side_orders_active
							.insert(order.id, order);
					}

					OrderPosition::SellSide(order) => {
						self.sell_side_orders_active
							.insert(order.id, order);
					}
				}
			}

			OrderStatus::Pending(order_position) => {
				match order_position {
					OrderPosition::BuySide(order) => {
						self.buy_side_orders_pending
							.insert(order.id, order);
					}

					OrderPosition::SellSide(order) => {
						self.sell_side_orders_pending
							.insert(order.id, order);
					}
				}
			}

			OrderStatus::Cancelled => todo!()
		}
	}

	pub fn get_buy_side_active_order(&self, order_id: &str) -> Order {



	}
}