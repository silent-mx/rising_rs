-- Your SQL goes here
CREATE TABLE sys_user (
    id UUID PRIMARY KEY DEFAULT generate_ulid (),
    username VARCHAR(50) UNIQUE NOT NULL, -- 用户名
    password VARCHAR(100) NOT NULL, -- 密码
    email VARCHAR(150) UNIQUE, -- 邮箱地址
    phone VARCHAR(11) UNIQUE, -- 手机号码
    nickname VARCHAR(50), -- 昵称
    gender Gender, -- 性别
    avatar TEXT, -- 头像地址
    is_admin BOOLEAN NOT NULL DEFAULT FALSE, -- 是否是系统管理员
    create_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    update_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 更新时间
    deleted_at TIMESTAMP WITH TIME ZONE -- 删除时间
);

SELECT diesel_manage_updated_at ('sys_user');