-- Add migration script here
CREATE TABLE IF NOT EXISTS subscriptions (
    subscription_id VARCHAR(26) PRIMARY KEY,
    user_name VARCHAR(20) NOT NULL,
    platform VARCHAR(50) NOT NULL,  
    subscription_token TEXT NOT NULL,
    status BOOLEAN DEFAULT TRUE, 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
