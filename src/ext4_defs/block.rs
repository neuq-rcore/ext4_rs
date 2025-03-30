use crate::prelude::*;

pub trait BlockDevice: Send + Sync + Any {
    fn read_offset(&self, offset: usize) -> Vec<u8>;
    fn write_offset(&self, offset: usize, data: &[u8]);
}

pub struct Block {
    pub disk_offset: usize,
    pub data: Vec<u8>,
}

impl Block {
    /// Load the block from the disk.
    pub fn load(block_device: Arc<dyn BlockDevice>, offset: usize) -> Self {
        let data = block_device.read_offset(offset);
        Block {
            disk_offset: offset,
            data,
        }
    }

    /// Load the block from inode block
    pub fn load_inode_root_block(data: &[u32; 15]) -> Self {
        let data_bytes: &[u8; 60] = unsafe {
            core::mem::transmute(data)
        };
        Block {
            disk_offset: 0, 
            data: data_bytes.to_vec(),
        }
    }

    /// Read the block as a specific type.
    pub fn read_as<T: Copy>(&self) -> T {
        self.read_offset_as(0)
    }

    /// Read the block as a specific type at a specific offset.
    pub fn read_offset_as<T: Copy>(&self, offset: usize) -> T {
        assert!(
            offset + core::mem::size_of::<T>() <= self.data.len(),
            "Read would overflow the block buffer"
        );

        let mut value = core::mem::MaybeUninit::<T>::uninit();

        unsafe {
            core::slice::from_raw_parts_mut(
                value.as_mut_ptr() as *mut u8,
                core::mem::size_of::<T>(),
            )
            .copy_from_slice(&self.data[offset..offset + core::mem::size_of::<T>()]);

            value.assume_init()
        }
    }

    /// Write data to the block starting at a specific offset.
    pub fn write_offset(&mut self, offset: usize, data: &[u8], len: usize) {
        let end = offset + len;
        if end <= self.data.len() {
            let slice_end = len.min(data.len());
            self.data[offset..end].copy_from_slice(&data[..slice_end]);
        } else {
            panic!("Write would overflow the block buffer");
        }
    }
}


impl Block{
    pub fn sync_blk_to_disk(&self, block_device: Arc<dyn BlockDevice>){
        block_device.write_offset(self.disk_offset, &self.data);
    }
}