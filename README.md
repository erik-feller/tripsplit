# TripSplit

An application to help split the financial cost of a trip between friends. The ultimate goal of the project is to have a way to easily record shared spending on costs such as gas and food that can be easily split later and simple amounts for reconciling differences can be given so that people can pay however they like. 

## To Do
* Add integration with data layer
  * Find UUID solution for users, trips and transactions
  * Decide how best to authenticate user info against trips
  * Test Postgres connector in Rust
  * Search for better timestamp implementation
* Add logic to verify incoming data
* Add logic to calculate trip reports and cost
* Create a UI
* Add sign on and user capabilities (Google auth first?)
* *stretch* add payment integration

