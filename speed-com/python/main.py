import time

def main():
    count = 0
    start_time = time.perf_counter() 
    for i in range(0, 1000000000):
        count = count + i
    
    endtime = time.perf_counter()
    execution_time = endtime - start_time
    
    print(f"Time Taken: {execution_time:.6f} seconds")
    print(count)


main()