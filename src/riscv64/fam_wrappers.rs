// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use vmm_sys_util::fam::{FamStruct, FamStructWrapper};

use riscv64::bindings::*;

// There is no constant in the kernel as far as the maximum number
// of registers on riscv, but KVM_GET_REG_LIST usually returns around 149.
// This could be verified by the following C program:
// #include <stdio.h>
// #include <stdlib.h>
// #include <fcntl.h>
// #include <sys/ioctl.h>
// #include <linux/kvm.h>

// int main() {
//     int kvm_fd, vm_fd, vcpu_fd;
//     struct kvm_reg_list *reg_list;
//     size_t mmap_size;

//     kvm_fd = open("/dev/kvm", O_RDWR | O_CLOEXEC);
//     if (kvm_fd == -1) {
//         perror("Failed to open /dev/kvm");
//         exit(1);
//     }

//     vm_fd = ioctl(kvm_fd, KVM_CREATE_VM, 0);
//     if (vm_fd == -1) {
//         perror("KVM_CREATE_VM");
//         exit(1);
//     }

//     mmap_size = ioctl(kvm_fd, KVM_GET_VCPU_MMAP_SIZE, 0);
//     if (mmap_size == -1) {
//         perror("KVM_GET_VCPU_MMAP_SIZE");
//         exit(1);
//     }

//     vcpu_fd = ioctl(vm_fd, KVM_CREATE_VCPU, 0);
//     if (vcpu_fd == -1) {
//         perror("KVM_CREATE_VCPU");
//         exit(1);
//     }

//     reg_list = (struct kvm_reg_list *)malloc(1024*1024*100);
//     reg_list->n = 500;
//     if (ioctl(vcpu_fd, KVM_GET_REG_LIST, reg_list) == -1) {
//         perror("KVM_GET_REG_LIST (get number of registers)");
//         exit(1);
//     }
//     int i = 0;
//     int count = 0;
//     for (i = 0; i < 500; i++) {
//         if (reg_list->reg[i] != 0) {
//             ++count;
//         }
//         printf("reg: %llu\n", reg_list->reg[i]);
//     }
//     printf("total regs: %d\n", count);

//     return 0;
// }
const RISCV64_REGS_MAX: usize = 500;

// Implement the FamStruct trait for kvm_reg_list.
generate_fam_struct_impl!(kvm_reg_list, u64, reg, u64, n, RISCV64_REGS_MAX);

// Implement the PartialEq trait for kvm_reg_list.
impl PartialEq for kvm_reg_list {
    fn eq(&self, other: &kvm_reg_list) -> bool {
        // No need to call entries's eq, FamStructWrapper's PartialEq will do it for you
        self.n == other.n
    }
}

/// Wrapper over the `kvm_reg_list` structure.
///
/// The `kvm_reg_list` structure contains a flexible array member. For details check the
/// [KVM API](https://www.kernel.org/doc/html/latest/virt/kvm/api.html#kvm-get-reg-list)
/// documentation on `kvm_reg_list`. To provide safe access to
/// the array elements, this type is implemented using
/// [FamStructWrapper](../vmm_sys_util/fam/struct.FamStructWrapper.html).
pub type RegList = FamStructWrapper<kvm_reg_list>;

#[cfg(test)]
mod tests {
    use super::RegList;

    #[test]
    fn test_reg_list_eq() {
        let mut wrapper = RegList::new(1).unwrap();
        assert_eq!(wrapper.as_slice().len(), 1);

        let mut wrapper2 = wrapper.clone();
        assert!(wrapper == wrapper2);

        wrapper.as_mut_slice()[0] = 1;
        assert!(wrapper != wrapper2);
        wrapper2.as_mut_slice()[0] = 1;
        assert!(wrapper == wrapper2);
    }
}
