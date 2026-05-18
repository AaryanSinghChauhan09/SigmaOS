/*
 * =============================================================================
 * Î£ SIGMAOS SHELL: SOVEREIGN JOB CALCULATOR SHARD (v3.0)
 * =============================================================================
 * Comprehensive calculation primitives for SigmaOS Profession Profiles.
 * Covers Finance, Engineering, Healthcare, Agriculture, Aviation, Logistics,
 * Retail, IT, and specialized sciences.
 * =============================================================================
 */

#include "sigma_kernel_types.h"

/* =============================================================================
 * 1. FINANCE & ACCOUNTING (Accountant, Banker, Trader, Entrepreneur)
 * ============================================================================= */

sigma_u64 calc_simple_interest(sigma_u64 principal, sigma_u64 rate_percent, sigma_u64 time_years) {
    return (principal * rate_percent * time_years) / 100;
}

sigma_u64 calc_roi(sigma_u64 net_profit, sigma_u64 total_investment) {
    if (total_investment == 0) return 0;
    return (net_profit * 100) / total_investment;
}

sigma_u64 calc_straight_line_depreciation(sigma_u64 asset_cost, sigma_u64 salvage_value, sigma_u64 useful_life_years) {
    if (useful_life_years == 0) return 0;
    return (asset_cost - salvage_value) / useful_life_years;
}

sigma_u64 calc_break_even_units(sigma_u64 fixed_costs, sigma_u64 sales_price_per_unit, sigma_u64 variable_cost_per_unit) {
    if (sales_price_per_unit <= variable_cost_per_unit) return 0;
    return fixed_costs / (sales_price_per_unit - variable_cost_per_unit);
}

sigma_u64 calc_working_capital(sigma_u64 current_assets, sigma_u64 current_liabilities) {
    return current_assets - current_liabilities;
}

/* =============================================================================
 * 2. HEALTHCARE (Doctor, Nurse, Pharmacist, Veterinarian)
 * ============================================================================= */

sigma_u64 calc_bmi(sigma_u64 weight_kg, sigma_u64 height_cm) {
    if (height_cm == 0) return 0;
    // BMI = weight(kg) / height(m)^2 = (weight * 10000) / (height_cm * height_cm)
    return (weight_kg * 10000) / (height_cm * height_cm);
}

sigma_u64 calc_iv_drip_rate(sigma_u64 volume_ml, sigma_u64 drop_factor, sigma_u64 time_minutes) {
    if (time_minutes == 0) return 0;
    return (volume_ml * drop_factor) / time_minutes; // drops per minute
}

sigma_u64 calc_pediatric_dosage_clark(sigma_u64 child_weight_lbs, sigma_u64 adult_dose) {
    return (child_weight_lbs * adult_dose) / 150;
}

sigma_u64 calc_fluid_resuscitation_parkland(sigma_u64 weight_kg, sigma_u64 burn_percentage) {
    return 4 * weight_kg * burn_percentage; // Total fluid in mL for 24h
}

/* =============================================================================
 * 3. ENGINEERING (Civil, Mechanical, Electrical, Architect)
 * ============================================================================= */

sigma_u64 calc_concrete_volume(sigma_u64 length_cm, sigma_u64 width_cm, sigma_u64 depth_cm) {
    // Returns cubic meters * 1000000 for precision
    return (length_cm * width_cm * depth_cm);
}

sigma_u64 calc_electrical_voltage_drop(sigma_u64 current, sigma_u64 distance, sigma_u64 cable_resistance_per_m) {
    return (2 * current * distance * cable_resistance_per_m);
}

sigma_u64 calc_mechanical_torque(sigma_u64 force_newtons, sigma_u64 radius_meters) {
    return force_newtons * radius_meters;
}

sigma_u64 calc_gear_ratio(sigma_u64 teeth_driven, sigma_u64 teeth_driver) {
    if (teeth_driver == 0) return 0;
    return (teeth_driven * 100) / teeth_driver; // x100 for percentage/ratio
}

/* =============================================================================
 * 4. AGRICULTURE & FARMING (Farmer, Agricultural Scientist)
 * ============================================================================= */

sigma_u64 calc_crop_yield_per_hectare(sigma_u64 total_yield_kg, sigma_u64 area_sq_meters) {
    if (area_sq_meters == 0) return 0;
    return (total_yield_kg * 10000) / area_sq_meters; 
}

sigma_u64 calc_plant_population(sigma_u64 area_sq_m, sigma_u64 row_spacing_m, sigma_u64 plant_spacing_m) {
    if (row_spacing_m == 0 || plant_spacing_m == 0) return 0;
    return area_sq_m / (row_spacing_m * plant_spacing_m);
}

/* =============================================================================
 * 5. AVIATION (Pilot, Aerospace Engineer)
 * ============================================================================= */

sigma_u64 calc_descent_rate(sigma_u64 ground_speed_knots, sigma_u64 descent_angle_degrees) {
    // Rule of thumb: rate of descent = ground speed * 5 (for 3 degree glide slope)
    return ground_speed_knots * 5; 
}

sigma_u64 calc_fuel_burn(sigma_u64 flight_time_mins, sigma_u64 burn_rate_per_hour) {
    return (flight_time_mins * burn_rate_per_hour) / 60;
}

/* =============================================================================
 * 6. LOGISTICS, RETAIL & SUPPLY CHAIN (Manager, Cashier)
 * ============================================================================= */

sigma_u64 calc_inventory_turnover(sigma_u64 cogs, sigma_u64 avg_inventory) {
    if (avg_inventory == 0) return 0;
    return cogs / avg_inventory;
}

sigma_u64 calc_freight_volumetric_weight(sigma_u64 l_cm, sigma_u64 w_cm, sigma_u64 h_cm, sigma_u64 dim_factor) {
    if (dim_factor == 0) dim_factor = 5000; // Standard international dim factor
    return (l_cm * w_cm * h_cm) / dim_factor;
}

sigma_u64 calc_retail_margin(sigma_u64 selling_price, sigma_u64 cost_price) {
    if (selling_price == 0) return 0;
    return ((selling_price - cost_price) * 100) / selling_price;
}

/* =============================================================================
 * 7. IT & CYBERSECURITY (Software Dev, Cybersecurity Analyst)
 * ============================================================================= */

sigma_u64 calc_network_transfer_time_seconds(sigma_u64 file_size_mb, sigma_u64 bandwidth_mbps) {
    if (bandwidth_mbps == 0) return 0;
    return (file_size_mb * 8) / bandwidth_mbps;
}

sigma_u64 calc_raid5_capacity(sigma_u64 drive_capacity_tb, sigma_u64 num_drives) {
    if (num_drives <= 1) return 0;
    return drive_capacity_tb * (num_drives - 1);
}

/* =============================================================================
 * 8. HUMAN RESOURCES & MARKETING (HR Manager, Marketing Pro)
 * ============================================================================= */

sigma_u64 calc_employee_turnover_rate(sigma_u64 employees_left, sigma_u64 avg_total_employees) {
    if (avg_total_employees == 0) return 0;
    return (employees_left * 100) / avg_total_employees;
}

sigma_u64 calc_customer_acquisition_cost(sigma_u64 total_marketing_spend, sigma_u64 new_customers_acquired) {
    if (new_customers_acquired == 0) return 0;
    return total_marketing_spend / new_customers_acquired;
}

/* =============================================================================
 * 9. CULINARY ARTS (Chef)
 * ============================================================================= */

sigma_u64 calc_bakers_percentage(sigma_u64 ingredient_weight, sigma_u64 flour_weight) {
    if (flour_weight == 0) return 0;
    return (ingredient_weight * 100) / flour_weight;
}

sigma_u64 calc_recipe_conversion_factor(sigma_u64 desired_yield, sigma_u64 original_yield) {
    if (original_yield == 0) return 0;
    return (desired_yield * 100) / original_yield; // Returns percentage multiplier
}

void sigma_job_calc_init() {
    kprintf("Î£ [JOB-CALC]: Sovereign Job Calculators active for 40+ professions.\n");
}

