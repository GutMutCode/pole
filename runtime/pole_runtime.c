// Pole Runtime Library
// Helper functions for low-level operations

#include <stdint.h>
#include <string.h>

// Pointer reading helpers

// Read 32-bit integer from pointer
int32_t pole_read_i32(void* ptr) {
    return *(int32_t*)ptr;
}

// Read 32-bit integer at byte offset
int32_t pole_read_i32_at(void* ptr, int offset) {
    return *(int32_t*)((char*)ptr + offset);
}

// Read 64-bit integer from pointer
int64_t pole_read_i64(void* ptr) {
    return *(int64_t*)ptr;
}

// Read 64-bit integer at byte offset
int64_t pole_read_i64_at(void* ptr, int offset) {
    return *(int64_t*)((char*)ptr + offset);
}

// Read pointer from pointer
void* pole_read_ptr(void* ptr) {
    return *(void**)ptr;
}

// Read pointer at byte offset
void* pole_read_ptr_at(void* ptr, int offset) {
    return *(void**)((char*)ptr + offset);
}

// Pointer writing helpers

// Write 32-bit integer to pointer
void pole_write_i32(void* ptr, int32_t value) {
    *(int32_t*)ptr = value;
}

// Write 32-bit integer at byte offset
void pole_write_i32_at(void* ptr, int offset, int32_t value) {
    *(int32_t*)((char*)ptr + offset) = value;
}

// Write 16-bit integer at byte offset
void pole_write_i16_at(void* ptr, int offset, int16_t value) {
    *(int16_t*)((char*)ptr + offset) = value;
}

// Write 64-bit integer to pointer
void pole_write_i64(void* ptr, int64_t value) {
    *(int64_t*)ptr = value;
}

// Write pointer to pointer
void pole_write_ptr(void* ptr, void* value) {
    *(void**)ptr = value;
}

// String helpers

// Convert C string pointer to Pole string (just returns the pointer as-is)
// In Pole, String is represented as char* at runtime
char* pole_ptr_to_string(void* ptr) {
    return (char*)ptr;
}
