fn main(){
	let num:i32 = 5;
	mutate_num_to_zero(num);
}
fn mutate_num_to_zero(mut param_num: i32){
	param_num = param_num*0;
	println!("param_num value :{}",param_num );
}