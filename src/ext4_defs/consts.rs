use bitflags::bitflags;

pub const BLOCK_SIZE: usize = 4096;

pub type Ext4Lblk = u32;
pub type Ext4Fsblk = u64;

pub const EOK: usize = 0;

/// Inode
pub const ROOT_INODE: u32 = 2;                      // 根目录 inode
pub const JOURNAL_INODE: u32 = 8;                   // 日志文件 inode
pub const UNDEL_DIR_INODE: u32 = 6;                 // 未删除目录 inode
pub const LOST_AND_FOUND_INODE: u32 = 11;           // lost+found 目录 inode
pub const EXT4_INODE_MODE_FILE: usize = 0x8000;
pub const EXT4_INODE_MODE_TYPE_MASK: u16 = 0xF000;
pub const EXT4_INODE_BLOCK_SIZE: usize = 512;
pub const EXT4_GOOD_OLD_INODE_SIZE: u16 = 128;
pub const EXT4_INODE_FLAG_EXTENTS: usize = 0x00080000; /* Inode uses extents */

/// Extent
pub const EXT_INIT_MAX_LEN: u16 = 32768;
pub const EXT_UNWRITTEN_MAX_LEN: u16 = 65535;
pub const EXT_MAX_BLOCKS: Ext4Lblk = core::u32::MAX;
pub const EXT4_EXTENT_MAGIC: u16 = 0xF30A;

/// BLock group descriptor flags.
pub const EXT4_MIN_BLOCK_GROUP_DESCRIPTOR_SIZE: u16 = 32;
pub const EXT4_MAX_BLOCK_GROUP_DESCRIPTOR_SIZE: u16 = 64;

/// SuperBlock
pub const SUPERBLOCK_OFFSET: usize = 1024;
pub const EXT4_SUPERBLOCK_OS_HURD: u32 = 1;