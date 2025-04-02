/* The Rafinery Optimization Problem */
/* Author: Bartosz Michalak */

param cost_to_buy_B1;
param cost_to_buy_B2;
param cost_of_destilation;
param cost_of_cracking;

param planned_benzin_production;
param planned_oil_production;
param planned_heavy_oil_benzin_production;

param accepted_sulfur_level;
param destilate_B1_benzin_sulfur_level;
param destilate_B2_benzin_sulfur_level;
param cracking_B1_benzin_sulfur_level;
param cracking_B2_benzin_sulfur_level;

# Decision variables: Amount of B1 Crude Oil purchased
var B1 >= 0;
# Decision variables: Amount of B2 Crude Oil purchased
var B2 >= 0;

# Decision variables: Amount of B1 Destilate sent to cracking
var B1_Destilate >= 0;
# Decision variables: Amount of B2 Destilate sent to cracking
var B2_Destilate >= 0;

# Decision variables: Amount of B1 Oil used for home oil
var B1_Used_For_Home_Oil >= 0;
# Decision variables: Amount of B2 Oil used for home oil
var B2_Used_For_Home_Oil >= 0;

# Objective: Minimize Total Cost of the process
minimize Total_Cost:
    B1 * (cost_to_buy_B1 + cost_of_destilation) +
    B2 * (cost_to_buy_B2 + cost_of_destilation) +
    B1_Destilate * cost_of_cracking +
    B2_Destilate * cost_of_cracking;

# Constraint: Amount of B1 Destilate sent to cracking cannot exceed 15% of B1 crude oil
s.t. amount_of_B1_Destilate:
  B1 * 0.15 >= B1_Destilate;

# Constraint: Amount of B2 Destilate sent to cracking cannot exceed 20% of B2 crude oil
s.t. amount_of_B2_Destilate:
  B2 * 0.2 >= B2_Destilate;

# Constraint: Amount of B1 Destilate used for home oil cannot exceed 40% of B1 crude oil
s.t. amount_of_B1_Used_For_Home_Oil:
  B1 * 0.4 >= B1_Used_For_Home_Oil;

# Constraint: Amount of B2 Destilate used for home oil cannot exceed 35% of B2 crude oil
s.t. amount_of_B2_Used_For_Home_Oil:
  B2 * 0.35 >= B2_Used_For_Home_Oil;

# Constraint: Total benzine production must be at least planned amount
s.t. benzin_production:
    B1 * 0.15 + B2 * 0.1 + (B1_Destilate + B2_Destilate) * 0.5 >= planned_benzin_production;

# Constraint: Total oil production must be at least planned amount
s.t. oil_production:
    B1_Used_For_Home_Oil + B2_Used_For_Home_Oil + (B1_Destilate + B2_Destilate) * 0.2 >= planned_oil_production;

# Constraint: Total heavy oil benzin production must be at least planned amount
s.t. heavy_oil_benzin_production:
    B1 * 0.15 + B2 * 0.25 +
    (B1_Destilate + B2_Destilate) * 0.06 +
    (B1 * 0.15 - B1_Destilate) + (B2 * 0.2 - B2_Destilate) +
    B1 * 0.4 - B1_Used_For_Home_Oil +
    B2 * 0.35 - B2_Used_For_Home_Oil >= planned_heavy_oil_benzin_production;

# Constraint: Total sulfur level in benzine must be below accepted level
s.t. sulfur_level:
    ( B1_Used_For_Home_Oil * destilate_B1_benzin_sulfur_level +
      B2_Used_For_Home_Oil * destilate_B2_benzin_sulfur_level +
      B1_Destilate * cracking_B1_benzin_sulfur_level +
      B2_Destilate * cracking_B2_benzin_sulfur_level
    )  <= accepted_sulfur_level * (
      B1_Used_For_Home_Oil + B2_Used_For_Home_Oil + B1_Destilate + B2_Destilate
    );

solve;

printf "Buy %.2f B1 crude oil\n", B1;
printf "Buy %.2f B2 crude oil\n", B2;
printf "Further destilate %.2f B1 destilate\n", B1_Destilate;
printf "Further destilate %.2f B2 destilate\n", B2_Destilate;

printf "Benzin produced %.2f \n", B1 * 0.15 + B2 * 0.1 + (B1_Destilate + B2_Destilate) * 0.5;
printf "Home Oil produced %.2f \n", B1_Used_For_Home_Oil + B2_Used_For_Home_Oil + (B1_Destilate + B2_Destilate) * 0.2 ;
printf "Heavy Oil produced %.2f \n",  B1 * 0.15 + B2 * 0.25 +
    (B1_Destilate + B2_Destilate) * 0.06 +
    (B1 * 0.15 - B1_Destilate) + (B2 * 0.2 - B2_Destilate) +
    B1 * 0.4 - B1_Used_For_Home_Oil +
    B2 * 0.35 - B2_Used_For_Home_Oil;

printf "Total_Cost %.2f\n", Total_Cost;

end;
