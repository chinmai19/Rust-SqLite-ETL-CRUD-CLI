import time
import psutil


def fib(n):
    if n <= 1:
        return n
    else:
        return fib(n-1)+fib(n-2)

if __name__ == "__main__":
    start_time = time.time()
    process = psutil.Process()
    cpu_before = process.cpu_percent()
    result = fib(35)
    cpu_after = process.cpu_percent()
    end_time = time.time()

    memory_info = psutil.virtual_memory()

    print("35th Fibonacci number: {}".format(result))
    print("Memory Usage: {:.2f}%".format((memory_info.total - memory_info.available) / float(memory_info.total) * 100.0))
    print("CPU used: {:.2f}%".format(cpu_after - cpu_before))
    print("Time taken: {:.6f} seconds".format(end_time - start_time))
