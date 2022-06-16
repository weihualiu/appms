-- 主版本号管理表
create table master_version (
    id integer PRIMARY KEY, -- 自增序号
    vernum integer not null, --
    vername varchar(20) not null, -- 字符串 2.0.0
    active varchar(1) not null, -- 激活标识
    pgid integer not null, -- 项目编号
);

-- 项目管理表
create table program (
    id integer PRIMARY KEY,
    en_name varchar(20) not null, -- 英文名称
    cn_name varchar(255) not null, -- 中文名称
);

-- 版本表
create table versions (
    id integer not null, -- 根据主版本号区分后自增
    mvid integer not null, -- 主版本号
    active varchar(1) not null, -- 激活生效
    PRIMARY KEY (id, mvid),
);

-- 增量文件表
create table patch_file (
    id integer primary key,
    filename varchar(255) not null, --
    src_version integer not null, --
    dst_version integer not null, --
);

-- 外部指令进度表
create table cmd_process (
    id integer primary key,
    perecnt integer not null, -- 进度: 0未开始,1进行中,2完成
    cmd varchar(255) not null, -- 指令内容
);
