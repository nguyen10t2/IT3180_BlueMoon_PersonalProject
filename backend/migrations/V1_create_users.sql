CREATE TYPE relationship AS ENUM (
    'ChuHo', 'Vo', 'Chong', 'Con', 'Cha', 'Me',
    'Ong', 'Ba', 'Anh', 'Chi', 'Em', 'Khac'
);
CREATE TYPE resident_status AS ENUM (
    'ThuongTru', 'TamTru', 'TamVang'
);
CREATE TYPE gender AS ENUM ('male', 'female', 'other');
CREATE TYPE user_role AS ENUM ('admin', 'manager', 'resident');
CREATE TYPE user_status AS ENUM ('active', 'inactive');
CREATE TABLE residents (
    resident_id SERIAL PRIMARY KEY,
    house_id INTEGER NOT NULL,
    fullname VARCHAR(100) NOT NULL,
    birth DATE,
    gender gender DEFAULT 'other',
    relationship relationship DEFAULT 'ChuHo',
    phone_number VARCHAR(15),
    occupation VARCHAR(15),
    resident_status resident_status DEFAULT 'ThuongTru',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    fullname VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    role user_role DEFAULT 'resident',
    status user_status DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    resident_id INT REFERENCES residents(resident_id)
);