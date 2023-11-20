// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (id) {
        id -> Integer,
        #[max_length = 100]
        address -> Nullable<Varchar>,
        #[max_length = 100]
        type_address -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Nullable<Varchar>,
        #[max_length = 100]
        state -> Nullable<Varchar>,
        #[max_length = 100]
        city -> Nullable<Varchar>,
        #[max_length = 100]
        zip -> Nullable<Varchar>,
        #[max_length = 100]
        lat -> Nullable<Varchar>,
        #[max_length = 100]
        long -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    allergens (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 200]
        description -> Nullable<Varchar>,
        #[max_length = 200]
        icon -> Nullable<Varchar>,
        store_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    branch_day_hours (id) {
        id -> Integer,
        branch_id -> Nullable<Integer>,
        day_hour_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    branch_delivery_settings (id) {
        id -> Integer,
        min_order_amount -> Nullable<Integer>,
        branch_id -> Nullable<Integer>,
        active -> Nullable<Tinyint>,
        max_distance -> Nullable<Integer>,
        delivery_cost -> Nullable<Decimal>,
        max_days_advance -> Nullable<Integer>,
        delivery_increment -> Nullable<Decimal>,
        allow_asap -> Nullable<Tinyint>,
        #[max_length = 100]
        delivery_type -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    branch_genders (id) {
        id -> Integer,
        branch_id -> Integer,
        gender_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    branch_phones (id) {
        id -> Integer,
        branch_id -> Nullable<Integer>,
        phone_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    branch_photos (id) {
        id -> Integer,
        photo_id -> Integer,
        branch_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    branch_pickup_settings (id) {
        id -> Integer,
        min_order_amount -> Nullable<Decimal>,
        is_active_curbside -> Nullable<Integer>,
        is_active_curbside_parking -> Nullable<Integer>,
        is_active_dine_in -> Nullable<Integer>,
        branch_id -> Nullable<Integer>,
        price_curbside_parking -> Nullable<Decimal>,
        pickup_increment -> Nullable<Decimal>,
        allow_asap -> Nullable<Bool>,
        max_days_advance -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    branch_promotion (id) {
        id -> Integer,
        branch_id -> Integer,
        promotion_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    branch_settings (id) {
        id -> Integer,
        hour_advance_edit_cancel_order -> Nullable<Integer>,
        branch_id -> Nullable<Integer>,
        max_item_per_order -> Nullable<Integer>,
        concurrent_items -> Nullable<Integer>,
        max_items_selected_per_order -> Nullable<Integer>,
        is_active_delivering -> Nullable<Tinyint>,
        is_active_pickup -> Nullable<Tinyint>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    branches (id) {
        id -> Integer,
        store_id -> Integer,
        #[max_length = 80]
        name -> Varchar,
        #[max_length = 255]
        presentation -> Nullable<Varchar>,
        address_id -> Integer,
        is_active -> Nullable<Tinyint>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    car_infos (id) {
        id -> Integer,
        #[max_length = 10]
        color -> Nullable<Varchar>,
        #[max_length = 10]
        model -> Nullable<Varchar>,
        #[max_length = 100]
        identifier -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    credit_cards (id) {
        id -> Integer,
        #[max_length = 15]
        card_number -> Nullable<Varchar>,
        id_customer -> Integer,
        #[max_length = 6]
        expiration_date -> Nullable<Varchar>,
        #[max_length = 100]
        card_name -> Varchar,
        cvc -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    customer_addresses (id) {
        id -> Integer,
        customer_id -> Integer,
        address_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    customer_authentications (id) {
        id -> Integer,
        session_start -> Nullable<Datetime>,
        session_end -> Nullable<Datetime>,
        #[max_length = 30]
        authentication_type -> Nullable<Varchar>,
        customer_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    customer_phones (id) {
        id -> Integer,
        customer_id -> Integer,
        phone_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    customer_photos (id) {
        id -> Integer,
        customer_id -> Nullable<Integer>,
        photo_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    customers (id) {
        id -> Integer,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 15]
        age -> Nullable<Varchar>,
        #[max_length = 50]
        gender -> Nullable<Varchar>,
        branch_id -> Integer,
        #[max_length = 15]
        type_customer -> Varchar,
        failed_login_attempts -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    customs (id) {
        id -> Integer,
        item_size_id -> Nullable<Integer>,
        #[max_length = 100]
        preference -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    day_hour_promotion (id) {
        id -> Integer,
        day_hour_id -> Integer,
        promotion_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    day_hours (id) {
        id -> Integer,
        #[max_length = 50]
        day -> Nullable<Varchar>,
        start_time -> Time,
        end_time -> Time,
        #[max_length = 100]
        type_of -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    default_group_modifiers (id) {
        id -> Integer,
        menu_category_id -> Nullable<Integer>,
        modifier_group_id -> Nullable<Integer>,
        modifier_item_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    delivery_details (id) {
        id -> Integer,
        order_type_delivery_id -> Nullable<Integer>,
        delivery_man_id -> Nullable<Integer>,
        package_pickup_time -> Nullable<Time>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    delivery_man (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        delivery_vehicle_id -> Nullable<Integer>,
        #[max_length = 100]
        driver_licence -> Nullable<Varchar>,
        #[max_length = 100]
        accident_insurance -> Nullable<Varchar>,
        delivery_number -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    delivery_vehicles (id) {
        id -> Integer,
        #[max_length = 100]
        vehicle_type -> Nullable<Varchar>,
        #[max_length = 100]
        identity -> Nullable<Varchar>,
        #[max_length = 100]
        color -> Nullable<Varchar>,
        #[max_length = 100]
        capacity -> Nullable<Varchar>,
        #[max_length = 100]
        mark -> Nullable<Varchar>,
        #[max_length = 100]
        model -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    delivery_zones (id) {
        id -> Integer,
        branch_id -> Nullable<Integer>,
        menu_type_id -> Nullable<Integer>,
        #[max_length = 100]
        type_delivery_zone -> Nullable<Varchar>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        distance_km -> Nullable<Decimal>,
        polygon -> Nullable<Text>,
        price -> Nullable<Decimal>,
        min_amount -> Nullable<Decimal>,
        max_amount -> Nullable<Decimal>,
        #[max_length = 100]
        notes -> Nullable<Varchar>,
        allow_asap -> Nullable<Tinyint>,
        asap_price_inc -> Nullable<Decimal>,
        min_delivery_time -> Nullable<Time>,
        max_delivery_time -> Nullable<Time>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    favorite_branches (id) {
        id -> Integer,
        branch_id -> Integer,
        customer_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    forbidden_words (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    genders (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    infractions (id) {
        id -> Integer,
        store_id -> Nullable<Integer>,
        forbidden_word_id -> Nullable<Integer>,
        created -> Nullable<Date>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    item_allergen (id) {
        id -> Integer,
        item_id -> Integer,
        allergen_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    item_modifier_groups (id) {
        id -> Integer,
        modifier_group_id -> Nullable<Integer>,
        menu_section_id -> Integer,
        items_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    item_photos (id) {
        id -> Integer,
        photo_id -> Integer,
        item_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        #[max_length = 80]
        name -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 255]
        max_capacity_day -> Varchar,
        most_liked -> Nullable<Decimal>,
        enabled_for_menu -> Nullable<Tinyint>,
        is_active -> Nullable<Integer>,
        restricted_18 -> Nullable<Tinyint>,
        estimated_minutes_item_preparation -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    items_promotios (id) {
        id -> Integer,
        item_id -> Nullable<Integer>,
        promotion_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    items_sizes (id) {
        id -> Integer,
        size_id -> Nullable<Integer>,
        item_id -> Nullable<Integer>,
        price -> Nullable<Decimal>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    list_addition_customs (id) {
        id -> Integer,
        item_id -> Nullable<Integer>,
        custom_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    loyalties (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        state -> Nullable<Tinyint>,
        store_id -> Nullable<Integer>,
        min_amount_consumption -> Nullable<Integer>,
        points_value -> Nullable<Integer>,
        max_points_to_use -> Nullable<Integer>,
        min_points_to_use -> Nullable<Integer>,
        #[max_length = 10]
        unit_value_points -> Nullable<Varchar>,
        value_conversion -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    loyalty_balances (id) {
        id -> Integer,
        loyalty_customer_id -> Integer,
        order_id -> Integer,
        points_earn -> Nullable<Integer>,
        point_lost -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    loyalty_customers (id) {
        id -> Integer,
        loyalty_id -> Integer,
        customer_id -> Integer,
        reward_points -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    loyalty_discounts (id) {
        id -> Integer,
        points -> Nullable<Integer>,
        #[max_length = 10]
        unit_value_points -> Nullable<Varchar>,
        discount -> Integer,
        order_id -> Integer,
        loyalty_customer_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_categories (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        menu_section_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_categories_items (id) {
        id -> Integer,
        item_id -> Nullable<Integer>,
        menu_category_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_categories_promotions (id) {
        id -> Integer,
        menu_category_id -> Nullable<Integer>,
        promotion_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_order_types (id) {
        id -> Integer,
        order_type_id -> Nullable<Integer>,
        menu_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_promotion (id) {
        id -> Integer,
        menu_id -> Integer,
        promotion_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_sections (id) {
        id -> Integer,
        #[max_length = 10]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        menu_id -> Integer,
        is_active -> Nullable<Tinyint>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_sections_hours (id) {
        id -> Integer,
        day_hour_id -> Nullable<Integer>,
        menu_section_id -> Nullable<Integer>,
        special_date -> Nullable<Date>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_sections_promotions (id) {
        id -> Integer,
        menu_section_id -> Integer,
        promotion_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_types (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        menu_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menu_types_config (id) {
        id -> Integer,
        min_amount_required -> Nullable<Decimal>,
        min_time_required -> Nullable<Integer>,
        #[max_length = 100]
        unit_time -> Nullable<Varchar>,
        menu_types_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    menus (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 200]
        description -> Nullable<Varchar>,
        branch_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    modifier_group (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 200]
        description -> Nullable<Varchar>,
        availability -> Nullable<Tinyint>,
        minimum -> Nullable<Integer>,
        maximum -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    modifier_group_items (id) {
        id -> Integer,
        modifier_group_id -> Nullable<Integer>,
        item_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    modifier_price (id) {
        id -> Integer,
        item_size_id -> Integer,
        modifier_group_item_id -> Integer,
        price -> Nullable<Decimal>,
        menu_section_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    nutricional_tables (id) {
        id -> Integer,
        energy_kcl -> Nullable<Decimal>,
        full_fat -> Nullable<Decimal>,
        satured_fat -> Nullable<Decimal>,
        cholesterol -> Nullable<Decimal>,
        proteins -> Nullable<Decimal>,
        fiber -> Nullable<Decimal>,
        vinamin_a -> Nullable<Decimal>,
        vitamin_d -> Nullable<Decimal>,
        calcium -> Nullable<Decimal>,
        iron -> Nullable<Decimal>,
        sodium -> Nullable<Decimal>,
        item_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_credit_cards (id) {
        id -> Integer,
        credit_card_id -> Integer,
        order_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_detail_customer (id) {
        id -> Integer,
        customer_id -> Nullable<Integer>,
        order_detail_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_details (id) {
        id -> Integer,
        custom_id -> Nullable<Integer>,
        order_id -> Nullable<Integer>,
        quantity -> Nullable<Integer>,
        unit_price -> Nullable<Decimal>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_guest_configurations (id) {
        id -> Integer,
        amount_limit -> Nullable<Integer>,
        order_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_qr_payments (id) {
        id -> Integer,
        order_id -> Integer,
        qr_payment_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_reservations (id) {
        id -> Integer,
        reservation_id -> Nullable<Integer>,
        order_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_reviews (id) {
        id -> Integer,
        review_id -> Nullable<Integer>,
        order_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_status (id) {
        id -> Integer,
        order_id -> Integer,
        state_id -> Integer,
        date -> Datetime,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    order_types (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_types_deliveries (id) {
        id -> Integer,
        customer_address_id -> Nullable<Integer>,
        #[max_length = 100]
        phone -> Nullable<Varchar>,
        order_type_order_id -> Nullable<Integer>,
        time -> Nullable<Time>,
        date -> Nullable<Date>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_types_order (id) {
        id -> Integer,
        order_id -> Integer,
        order_type_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    order_types_pickups (id) {
        id -> Integer,
        datetime -> Nullable<Datetime>,
        car_info_id -> Nullable<Integer>,
        parking_reservation_id -> Nullable<Integer>,
        dine_in -> Nullable<Tinyint>,
        order_type_order_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        total_price -> Nullable<Decimal>,
        branch_id -> Integer,
        customer_id -> Integer,
        total_taxes -> Nullable<Decimal>,
        date -> Nullable<Timestamp>,
        #[max_length = 200]
        notes -> Nullable<Varchar>,
        amount -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
        amount_discount_loyalties -> Nullable<Integer>,
    }
}

diesel::table! {
    parameters_values (id) {
        id -> Integer,
        branch_id -> Integer,
        payment_parameters_id -> Integer,
        #[max_length = 100]
        value -> Nullable<Varchar>,
        #[max_length = 15]
        type_variable -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    parking_lots (id) {
        id -> Integer,
        branch_id -> Nullable<Integer>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        location -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    parking_reservations (id) {
        id -> Integer,
        parking_lot_id -> Nullable<Integer>,
        datetime -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    payment_gateway_orders (id) {
        id -> Integer,
        order_id -> Integer,
        payment_gateway_id -> Integer,
        datetime_process -> Nullable<Datetime>,
        response_server -> Nullable<Longtext>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    payment_gateways (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    payment_parameters (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        payment_gateway_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    permissions (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 30]
        code -> Nullable<Varchar>,
        #[max_length = 191]
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    phones (id) {
        id -> Integer,
        number -> Nullable<Integer>,
        region_code -> Nullable<Integer>,
        #[max_length = 100]
        type_of -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    photo_types (id) {
        id -> Integer,
        #[max_length = 30]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        size -> Nullable<Double>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    photos (id) {
        id -> Integer,
        #[max_length = 30]
        name -> Nullable<Varchar>,
        #[max_length = 200]
        path -> Nullable<Varchar>,
        #[max_length = 50]
        size -> Nullable<Varchar>,
        is_default -> Nullable<Tinyint>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    promotion_orders (id) {
        id -> Integer,
        order_id -> Nullable<Integer>,
        customer_id -> Nullable<Integer>,
        promotion_id -> Nullable<Integer>,
        #[max_length = 100]
        origin_promotion -> Nullable<Varchar>,
        #[max_length = 100]
        discount_type -> Nullable<Varchar>,
        amount_discount -> Nullable<Decimal>,
        date -> Nullable<Date>,
        #[max_length = 100]
        promotion_state -> Nullable<Varchar>,
        quantity_available_promotions -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    promotions (id) {
        id -> Integer,
        #[max_length = 50]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        type_promotion -> Nullable<Integer>,
        order_type -> Nullable<Integer>,
        start_date -> Nullable<Datetime>,
        valid_until -> Nullable<Datetime>,
        #[max_length = 100]
        discount_type -> Nullable<Varchar>,
        allowed_uses -> Nullable<Integer>,
        discount_value -> Nullable<Decimal>,
        min_order_amount -> Nullable<Integer>,
        max_discount_per_order -> Nullable<Integer>,
        is_active -> Nullable<Tinyint>,
        #[max_length = 100]
        cupon_code -> Nullable<Varchar>,
        #[max_length = 100]
        is_canceled_order -> Nullable<Varchar>,
        quantity_uses_per_person -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    qr_payments (id) {
        id -> Integer,
        #[max_length = 100]
        img_path -> Varchar,
        #[max_length = 100]
        entity_name -> Varchar,
        #[max_length = 100]
        code -> Nullable<Varchar>,
        branch_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    reservations (id) {
        id -> Integer,
        #[max_length = 100]
        code -> Nullable<Varchar>,
        customer_id -> Nullable<Integer>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        #[max_length = 100]
        state -> Nullable<Varchar>,
        creation_date -> Nullable<Date>,
        #[max_length = 100]
        more_info -> Nullable<Varchar>,
        start_time -> Nullable<Time>,
        end_time -> Nullable<Time>,
        resources_id -> Integer,
        branch_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    reservations_history (id) {
        id -> Integer,
        reservation_id -> Integer,
        #[max_length = 100]
        status_reservation -> Nullable<Varchar>,
        date_created -> Nullable<Date>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    resource_types (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    resources (id) {
        id -> Integer,
        resource_type_id -> Nullable<Integer>,
        branch_id -> Nullable<Integer>,
        max_capacity_people -> Nullable<Integer>,
        price -> Nullable<Decimal>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        number -> Nullable<Integer>,
        resource_configuration_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    resources_configurations (id) {
        id -> Integer,
        limit_reservation_per_user -> Nullable<Integer>,
        #[max_length = 15]
        unit_time -> Nullable<Varchar>,
        max_days_advance_reservation -> Nullable<Integer>,
        minutes_waiting_tolerance -> Nullable<Integer>,
        min_reservation_hours -> Nullable<Integer>,
        max_reservation_hours -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    restaurant (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        display_name -> Nullable<Varchar>,
        #[max_length = 100]
        address -> Varchar,
        #[max_length = 100]
        city -> Varchar,
        #[max_length = 2]
        state -> Char,
        #[max_length = 100]
        zip -> Varchar,
        #[max_length = 100]
        country -> Varchar,
        lat -> Decimal,
        lng -> Decimal,
        #[max_length = 512]
        description -> Nullable<Varchar>,
        #[max_length = 100]
        phone_no -> Varchar,
        #[max_length = 100]
        fax_no -> Nullable<Varchar>,
        #[max_length = 10]
        time_zone -> Varchar,
        #[max_length = 1]
        active -> Char,
        #[max_length = 1]
        inactive_reason -> Nullable<Char>,
        #[max_length = 255]
        time_zone_string -> Nullable<Varchar>,
        uploaded_logo -> Nullable<Text>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    reviews (id) {
        id -> Integer,
        puntuation -> Integer,
        date -> Datetime,
        #[max_length = 255]
        message -> Varchar,
        like -> Nullable<Integer>,
        branch_id -> Nullable<Integer>,
        customer_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 191]
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles_permissions (id) {
        id -> Integer,
        permission_id -> Nullable<Integer>,
        role_id -> Nullable<Integer>,
    }
}

diesel::table! {
    schedules (id) {
        id -> Integer,
        resource_id -> Nullable<Integer>,
        day_hour_id -> Nullable<Integer>,
        resource_type_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    sizes (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    states (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        description -> Varchar,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    stores (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        page_url -> Nullable<Varchar>,
        verified -> Nullable<Tinyint>,
        is_active -> Nullable<Tinyint>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    stores_type_of_coins (id) {
        id -> Integer,
        store_id -> Nullable<Integer>,
        type_of_coin_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    taxes (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 150]
        description -> Nullable<Varchar>,
        discount_amount -> Nullable<Decimal>,
        #[max_length = 100]
        amount_unit -> Nullable<Varchar>,
        branch_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    taxes_orders (id) {
        id -> Integer,
        order_id -> Nullable<Integer>,
        taxes_id -> Nullable<Integer>,
        amount_taxes -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    testing (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Integer,
        total_price_order -> Nullable<Decimal>,
        order_id -> Integer,
        date -> Date,
        credit_card_id -> Integer,
        customer_id -> Nullable<Integer>,
        #[max_length = 100]
        status -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    type_of_coins (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    user_phones (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        phone_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    user_photos (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        photo_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    user_role (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        role_id -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        store_id -> Nullable<Integer>,
        branch_id -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        login_session -> Varchar,
    }
}

diesel::joinable!(allergens -> stores (store_id));
diesel::joinable!(branch_day_hours -> branches (branch_id));
diesel::joinable!(branch_day_hours -> day_hours (day_hour_id));
diesel::joinable!(branch_delivery_settings -> branches (branch_id));
diesel::joinable!(branch_genders -> branches (branch_id));
diesel::joinable!(branch_genders -> genders (gender_id));
diesel::joinable!(branch_phones -> branches (branch_id));
diesel::joinable!(branch_phones -> phones (phone_id));
diesel::joinable!(branch_photos -> branches (branch_id));
diesel::joinable!(branch_photos -> photos (photo_id));
diesel::joinable!(branch_pickup_settings -> branches (branch_id));
diesel::joinable!(branch_promotion -> branches (branch_id));
diesel::joinable!(branch_promotion -> promotions (promotion_id));
diesel::joinable!(branch_settings -> branches (branch_id));
diesel::joinable!(branches -> addresses (address_id));
diesel::joinable!(branches -> stores (store_id));
diesel::joinable!(credit_cards -> customers (id_customer));
diesel::joinable!(customer_addresses -> addresses (address_id));
diesel::joinable!(customer_addresses -> customers (customer_id));
diesel::joinable!(customer_authentications -> customers (customer_id));
diesel::joinable!(customer_phones -> customers (customer_id));
diesel::joinable!(customer_phones -> phones (phone_id));
diesel::joinable!(customer_photos -> customers (customer_id));
diesel::joinable!(customer_photos -> photos (photo_id));
diesel::joinable!(customers -> branches (branch_id));
diesel::joinable!(customs -> items_sizes (item_size_id));
diesel::joinable!(day_hour_promotion -> day_hours (day_hour_id));
diesel::joinable!(day_hour_promotion -> promotions (promotion_id));
diesel::joinable!(default_group_modifiers -> items (modifier_item_id));
diesel::joinable!(default_group_modifiers -> menu_categories (menu_category_id));
diesel::joinable!(default_group_modifiers -> modifier_group (modifier_group_id));
diesel::joinable!(delivery_details -> delivery_man (delivery_man_id));
diesel::joinable!(delivery_details -> order_types_deliveries (order_type_delivery_id));
diesel::joinable!(delivery_man -> delivery_vehicles (delivery_vehicle_id));
diesel::joinable!(delivery_man -> users (user_id));
diesel::joinable!(delivery_zones -> branches (branch_id));
diesel::joinable!(delivery_zones -> menu_types (menu_type_id));
diesel::joinable!(favorite_branches -> branches (branch_id));
diesel::joinable!(favorite_branches -> customers (customer_id));
diesel::joinable!(infractions -> forbidden_words (forbidden_word_id));
diesel::joinable!(infractions -> stores (store_id));
diesel::joinable!(item_allergen -> allergens (allergen_id));
diesel::joinable!(item_allergen -> items (item_id));
diesel::joinable!(item_modifier_groups -> items (items_id));
diesel::joinable!(item_modifier_groups -> menu_sections (menu_section_id));
diesel::joinable!(item_modifier_groups -> modifier_group (modifier_group_id));
diesel::joinable!(item_photos -> items (item_id));
diesel::joinable!(item_photos -> photos (photo_id));
diesel::joinable!(items_promotios -> items (item_id));
diesel::joinable!(items_promotios -> promotions (promotion_id));
diesel::joinable!(items_sizes -> items (item_id));
diesel::joinable!(items_sizes -> sizes (size_id));
diesel::joinable!(list_addition_customs -> customs (custom_id));
diesel::joinable!(list_addition_customs -> items (item_id));
diesel::joinable!(loyalties -> stores (store_id));
diesel::joinable!(loyalty_balances -> loyalty_customers (loyalty_customer_id));
diesel::joinable!(loyalty_balances -> orders (order_id));
diesel::joinable!(loyalty_customers -> customers (customer_id));
diesel::joinable!(loyalty_customers -> loyalties (loyalty_id));
diesel::joinable!(loyalty_discounts -> loyalty_customers (loyalty_customer_id));
diesel::joinable!(loyalty_discounts -> orders (order_id));
diesel::joinable!(menu_categories -> menu_sections (menu_section_id));
diesel::joinable!(menu_categories_items -> items (item_id));
diesel::joinable!(menu_categories_items -> menu_categories (menu_category_id));
diesel::joinable!(menu_categories_promotions -> menu_categories (menu_category_id));
diesel::joinable!(menu_categories_promotions -> promotions (promotion_id));
diesel::joinable!(menu_order_types -> menus (menu_id));
diesel::joinable!(menu_order_types -> order_types (order_type_id));
diesel::joinable!(menu_promotion -> menus (menu_id));
diesel::joinable!(menu_promotion -> promotions (promotion_id));
diesel::joinable!(menu_sections -> menus (menu_id));
diesel::joinable!(menu_sections_hours -> day_hours (day_hour_id));
diesel::joinable!(menu_sections_hours -> menu_sections (menu_section_id));
diesel::joinable!(menu_sections_promotions -> menu_sections (menu_section_id));
diesel::joinable!(menu_sections_promotions -> promotions (promotion_id));
diesel::joinable!(menu_types -> menus (menu_id));
diesel::joinable!(menu_types_config -> menu_types (menu_types_id));
diesel::joinable!(menus -> branches (branch_id));
diesel::joinable!(modifier_group_items -> items (item_id));
diesel::joinable!(modifier_group_items -> modifier_group (modifier_group_id));
diesel::joinable!(modifier_price -> items_sizes (item_size_id));
diesel::joinable!(modifier_price -> menu_sections (menu_section_id));
diesel::joinable!(modifier_price -> modifier_group_items (modifier_group_item_id));
diesel::joinable!(nutricional_tables -> items (item_id));
diesel::joinable!(order_credit_cards -> credit_cards (credit_card_id));
diesel::joinable!(order_credit_cards -> orders (order_id));
diesel::joinable!(order_detail_customer -> customers (customer_id));
diesel::joinable!(order_detail_customer -> order_details (order_detail_id));
diesel::joinable!(order_details -> customs (custom_id));
diesel::joinable!(order_details -> orders (order_id));
diesel::joinable!(order_guest_configurations -> orders (order_id));
diesel::joinable!(order_qr_payments -> orders (order_id));
diesel::joinable!(order_qr_payments -> qr_payments (qr_payment_id));
diesel::joinable!(order_reservations -> orders (order_id));
diesel::joinable!(order_reservations -> reservations (reservation_id));
diesel::joinable!(order_reviews -> orders (order_id));
diesel::joinable!(order_reviews -> reviews (review_id));
diesel::joinable!(order_status -> orders (order_id));
diesel::joinable!(order_status -> states (state_id));
diesel::joinable!(order_types_deliveries -> customer_addresses (customer_address_id));
diesel::joinable!(order_types_deliveries -> order_types_order (order_type_order_id));
diesel::joinable!(order_types_order -> order_types (order_type_id));
diesel::joinable!(order_types_order -> orders (order_id));
diesel::joinable!(order_types_pickups -> car_infos (car_info_id));
diesel::joinable!(order_types_pickups -> order_types_order (order_type_order_id));
diesel::joinable!(order_types_pickups -> parking_reservations (parking_reservation_id));
diesel::joinable!(orders -> branches (branch_id));
diesel::joinable!(orders -> customers (customer_id));
diesel::joinable!(parameters_values -> branches (branch_id));
diesel::joinable!(parameters_values -> payment_parameters (payment_parameters_id));
diesel::joinable!(parking_lots -> branches (branch_id));
diesel::joinable!(parking_reservations -> parking_lots (parking_lot_id));
diesel::joinable!(payment_gateway_orders -> orders (order_id));
diesel::joinable!(payment_gateway_orders -> payment_gateways (payment_gateway_id));
diesel::joinable!(payment_parameters -> payment_gateways (payment_gateway_id));
diesel::joinable!(promotion_orders -> customers (customer_id));
diesel::joinable!(promotion_orders -> orders (order_id));
diesel::joinable!(promotion_orders -> promotions (promotion_id));
diesel::joinable!(qr_payments -> branches (branch_id));
diesel::joinable!(reservations -> branches (branch_id));
diesel::joinable!(reservations -> customers (customer_id));
diesel::joinable!(reservations -> resources (resources_id));
diesel::joinable!(reservations_history -> reservations (reservation_id));
diesel::joinable!(resources -> branches (branch_id));
diesel::joinable!(resources -> resource_types (resource_type_id));
diesel::joinable!(resources -> resources_configurations (resource_configuration_id));
diesel::joinable!(reviews -> branches (branch_id));
diesel::joinable!(reviews -> customers (customer_id));
diesel::joinable!(roles_permissions -> permissions (permission_id));
diesel::joinable!(roles_permissions -> roles (role_id));
diesel::joinable!(schedules -> day_hours (day_hour_id));
diesel::joinable!(schedules -> resource_types (resource_type_id));
diesel::joinable!(schedules -> resources (resource_id));
diesel::joinable!(stores_type_of_coins -> stores (store_id));
diesel::joinable!(stores_type_of_coins -> type_of_coins (type_of_coin_id));
diesel::joinable!(taxes -> branches (branch_id));
diesel::joinable!(taxes_orders -> orders (order_id));
diesel::joinable!(taxes_orders -> taxes (taxes_id));
diesel::joinable!(transactions -> credit_cards (credit_card_id));
diesel::joinable!(transactions -> customers (customer_id));
diesel::joinable!(transactions -> orders (order_id));
diesel::joinable!(user_phones -> phones (phone_id));
diesel::joinable!(user_phones -> users (user_id));
diesel::joinable!(user_photos -> photos (photo_id));
diesel::joinable!(user_photos -> users (user_id));
diesel::joinable!(user_role -> roles (role_id));
diesel::joinable!(user_role -> users (user_id));
diesel::joinable!(users -> branches (branch_id));
diesel::joinable!(users -> stores (store_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    allergens,
    branch_day_hours,
    branch_delivery_settings,
    branch_genders,
    branch_phones,
    branch_photos,
    branch_pickup_settings,
    branch_promotion,
    branch_settings,
    branches,
    car_infos,
    credit_cards,
    customer_addresses,
    customer_authentications,
    customer_phones,
    customer_photos,
    customers,
    customs,
    day_hour_promotion,
    day_hours,
    default_group_modifiers,
    delivery_details,
    delivery_man,
    delivery_vehicles,
    delivery_zones,
    favorite_branches,
    forbidden_words,
    genders,
    infractions,
    item_allergen,
    item_modifier_groups,
    item_photos,
    items,
    items_promotios,
    items_sizes,
    list_addition_customs,
    loyalties,
    loyalty_balances,
    loyalty_customers,
    loyalty_discounts,
    menu_categories,
    menu_categories_items,
    menu_categories_promotions,
    menu_order_types,
    menu_promotion,
    menu_sections,
    menu_sections_hours,
    menu_sections_promotions,
    menu_types,
    menu_types_config,
    menus,
    modifier_group,
    modifier_group_items,
    modifier_price,
    nutricional_tables,
    order_credit_cards,
    order_detail_customer,
    order_details,
    order_guest_configurations,
    order_qr_payments,
    order_reservations,
    order_reviews,
    order_status,
    order_types,
    order_types_deliveries,
    order_types_order,
    order_types_pickups,
    orders,
    parameters_values,
    parking_lots,
    parking_reservations,
    payment_gateway_orders,
    payment_gateways,
    payment_parameters,
    permissions,
    phones,
    photo_types,
    photos,
    promotion_orders,
    promotions,
    qr_payments,
    reservations,
    reservations_history,
    resource_types,
    resources,
    resources_configurations,
    restaurant,
    reviews,
    roles,
    roles_permissions,
    schedules,
    sizes,
    states,
    stores,
    stores_type_of_coins,
    taxes,
    taxes_orders,
    testing,
    transactions,
    type_of_coins,
    user_phones,
    user_photos,
    user_role,
    users,
);
