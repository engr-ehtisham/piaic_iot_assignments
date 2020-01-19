mod my_message
{
	 pub mod my_new_message
	{
		pub fn body()
		{
			println!("hello world!!!");

		}	
	}
}
fn main()
{
my_message::my_new_message::body();
}


