/* The Rafinery Optimization Problem */
/* Author: Bartosz Michalak */

/* Set of Destilation products */
set Destilation_products;

/* Set of Cracking products */
set Cracking_products;

/* Set of End products */
set End_products;

/* Cost in dolars for buying one ton of B1 crude oil type */
param cost_to_buy_B1;

/* Cost in dolars for buying one ton of B2 crude oil type */
param cost_to_buy_B2;

/* Cost to destilate one ton of crude oil */
param cost_of_destilation;

/* Cost to crack one ton of destilate */
param cost_of_cracking;


/* Planned Benzin production */
param planned_benzin_production;

/* Planned Oil production */
param planned_oil_production;

/* Planned Heavy-Oil-Benzin production */
param planned_heavy_oil_benzin_production;


/* Accepted sulfur level */
param accepted_sulfur_level;

/* Destilate B1 benzin sulfur level */
param destilate_B1_benzin_sulfur_level;

/* Destilate B2 benzin sulfur level */
param destilate_B2_benzin_sulfur_level;

/* Cracking B1 benzin sulfur level */
param cracking_B1_benzin_sulfur_level;

/* Cracking B2 benzin sulfur level */
param cracking_B2_benzin_sulfur_level;

/* Decision variables: Amount of Crude Oil purchased */
# var B1 integer >= 0;
# var B2 integer >= 0;
# # Zastanow sie czy musi byc int
# var B1_Destilate integer >= 0;
# var B2_Destilate integer >= 0;
# var B1_Used_For_Home_Oil integer >= 0;
# var B2_Used_For_Home_Oil integer >= 0;

var B1 >= 0;
var B2 >= 0;
# Zastanow sie czy musi byc int
var B1_Destilate >= 0;
var B2_Destilate >= 0;
var B1_Used_For_Home_Oil >= 0;
var B2_Used_For_Home_Oil >= 0;

minimize Total_Cost:
    B1 * (cost_to_buy_B1 + cost_of_destilation) +
    B2 * (cost_to_buy_B2 + cost_of_destilation) +
    B1_Destilate * cost_of_cracking +
    B2_Destilate * cost_of_cracking;


s.t. amount_of_B1_Destilate:
  B1 * 0.15 >= B1_Destilate;

s.t. amount_of_B2_Destilate:
  B2 * 0.2 >= B2_Destilate;

s.t. amount_of_B1_Used_For_Home_Oil:
  B1 * 0.4 >= B1_Used_For_Home_Oil;

s.t. amount_of_B2_Used_For_Home_Oil:
  B2 * 0.35 >= B2_Used_For_Home_Oil;

/* Supply constraints: Cannot transport more cranes than available */
s.t. benzin_production:
    B1 * 0.15 + B2 * 0.1 + (B1_Destilate + B2_Destilate) * 0.5 >= planned_benzin_production;

/* Supply constraints: Cannot transport more cranes than available */
s.t. oil_production:
    B1_Used_For_Home_Oil + B2_Used_For_Home_Oil + (B1_Destilate + B2_Destilate) * 0.2 >= planned_oil_production;

/* Supply constraints: Cannot transport more cranes than available */
s.t. heavy_oil_benzin_production:
    B1 * 0.15 + B2 * 0.25 +
    (B1_Destilate + B2_Destilate) * 0.06 +
    (B1 * 0.15 - B1_Destilate) + (B2 * 0.2 - B2_Destilate) +
    B1 * 0.4 - B1_Used_For_Home_Oil +
    B2 * 0.35 - B2_Used_For_Home_Oil >= planned_heavy_oil_benzin_production;


#TODO zastanow sie nad integerami, czy nie mozna tego ladniej, wartosci na siarke
/* Sulfur levels */
s.t. sulfur_level:
    ( B1_Used_For_Home_Oil * destilate_B1_benzin_sulfur_level +
      B2_Used_For_Home_Oil * destilate_B2_benzin_sulfur_level +
      B1_Destilate * cracking_B1_benzin_sulfur_level +
      B2_Destilate * cracking_B2_benzin_sulfur_level
    )  <= accepted_sulfur_level * (
      B1_Used_For_Home_Oil + B2_Used_For_Home_Oil + B1_Destilate + B2_Destilate
    );

/* Solve the model */
solve;

/* Print results */
printf "Buy %d B1 crude oil\n", B1;
printf "Buy %d B2 crude oil\n", B2;
printf "Further destilate %d B2 destilate\n", B1_Destilate;
printf "Further destilate %d B2 destilate\n", B2_Destilate;

printf "Benzin produced %d \n", B1 * 0.15 + B2 * 0.1 + (B1_Destilate + B2_Destilate) * 0.5;
printf "Home Oil produced %d \n", B1_Used_For_Home_Oil + B2_Used_For_Home_Oil + (B1_Destilate + B2_Destilate) * 0.2 ;
printf "Heavy Oil produced %d \n",  B1 * 0.15 + B2 * 0.25 +
    (B1_Destilate + B2_Destilate) * 0.06 +
    (B1 * 0.15 - B1_Destilate) + (B2 * 0.2 - B2_Destilate) +
    B1 * 0.4 - B1_Used_For_Home_Oil +
    B2 * 0.35 - B2_Used_For_Home_Oil;

printf "Total_Cost %.2f\n", Total_Cost;

end;
