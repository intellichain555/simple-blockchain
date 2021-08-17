use crate::hash_string;
use chrono;

pub struct Transaction {
  from_address: String,
  to_address: String,
  ammount: u64,
  timestamp: String,
  hash: String
}

pub struct Block {
  timestamp: String,
  ammount: u64,
  nonce: u64,
  hash: String,
  previous_hash: String,
  transactions: Vec<Transaction>
}

pub trait Printer {
  fn print(&self) -> String;
}

impl Transaction {
  pub fn new(
      from_address: String,
      to_address: String,
      ammount: u64) -> Transaction {
    let mut t: Transaction = Transaction {
      timestamp : format!("{:?}", chrono::offset::Utc::now()),
      ammount : ammount,
      from_address : from_address,
      to_address : to_address,
      hash : String::new()
    };
    t.calc_hash();
    return t;
  }

  pub fn calc_hash(&mut self) {
    let mut s: String = String::new();
    s.push_str(&self.from_address);
    s.push_str(&self.to_address);
    s.push_str(&self.ammount.to_string());
    s.push_str(&self.timestamp);
    self.hash = hash_string(&s);
  }
}

impl Block {
  pub fn new(
      transactions: Vec<Transaction>,
      ammount: u64) -> Block {
    let mut b: Block = Block {
      timestamp : format!("{:?}", chrono::offset::Utc::now()),
      ammount : ammount,
      nonce : 0,
      hash: String::new(),
      previous_hash: String::new(),
      transactions: transactions
    };
    b.calc_hash();
    return b;
  }

  pub fn get_transactions(&self) -> String {
    let mut s: String = String::new();
    s.push_str("[");
    for transaction in self.transactions.iter() {
      let t: String = transaction.print();
      if s.len() > 1 {
        s.push_str(", ");
      }
      s.push_str(&t);
    }
    s.push_str("]");
    return s;
  }

  pub fn calc_hash(&mut self) {
    let mut s: String = String::new();
    s.push_str(&self.get_transactions());
    s.push_str(&self.timestamp);
    s.push_str(&self.nonce.to_string());
    s.push_str(&self.ammount.to_string());
    s.push_str(&self.previous_hash);
    self.hash = hash_string(&s);
  }

  pub fn mine_block(&mut self, difficulty: usize) {
    let hash_compare: String = String::from_utf8(vec![b'0'; difficulty]).unwrap();
    let mut hash_start: String = self.hash.chars().take(difficulty).collect();
    while !hash_compare.eq(&hash_start) {
      self.nonce += 1;
      self.calc_hash();
      hash_start = self.hash.chars().take(difficulty).collect();
    }
  }
}

impl Printer for Block {
  fn print(&self) -> String {
    return format!(
      "[Transactions: {}, Timestamp: {}, Nonce: {}, Hash: {}, PreviousHash: {}]",
      self.get_transactions(), self.timestamp, self.nonce, self.hash, self.previous_hash);
  }
}

impl Printer for Transaction {
  fn print(&self) -> String {
    return format!("[From: {}, To: {}, Ammount: {}, Timestamp: {}, Hash: {}]",
      self.from_address, self.to_address, self.ammount,
      self.timestamp, self.hash);
  }
}


