-- CÁC LOẠI DỮ LIỆU ENUM (Đã giữ nguyên, Tốt)
CREATE TYPE operational_status AS ENUM (
    'active', 'inactive', 'temporarily_away'
);

CREATE TYPE relationship AS ENUM (
    'chusohuu', 'nguoidaidien', 'thanhvien', 'nguoithue'
);
CREATE TYPE gender AS ENUM ('male', 'female', 'other');
CREATE TYPE user_role AS ENUM ('admin', 'manager', 'resident');
CREATE TYPE status AS ENUM ('active', 'inactive');

CREATE TABLE houses (
    house_id SERIAL PRIMARY KEY,
    house_number VARCHAR(20) UNIQUE NOT NULL, 
    floor INT, 
    area NUMERIC(8, 2), 
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE residents (
    resident_id SERIAL PRIMARY KEY,
    house_id INT REFERENCES houses(house_id),
    fullname VARCHAR(100) NOT NULL,
    birth DATE,
    gender gender DEFAULT 'other',
    phone_number VARCHAR(15),
    relationship relationship DEFAULT 'chusohuu',
    residency_status operational_status DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    fullname VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    role user_role DEFAULT 'resident',
    status status DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    resident_id INT REFERENCES residents(resident_id)
);

CREATE TABLE refresh_tokens (
    id SERIAL PRIMARY KEY,
    token TEXT UNIQUE NOT NULL,
    user_id INT REFERENCES users(user_id) NOT NULL, 
    expires_at TIMESTAMP NOT NULL,
    revoked_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);