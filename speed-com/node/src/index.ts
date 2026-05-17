function main(){
    let count = 0;
    const startTime = performance.now(); // High-resolution timestamp
    
    for (let i = 0; i < 10000; i++){
        count = count + i;
        console.log(`count :${count}`)
    }
    
    const endTime = performance.now();
    console.log(count);
    
    const totalTimeSeconds = (endTime - startTime) / 1000;
    console.log(`Time Taken ${totalTimeSeconds.toFixed(4)} seconds`);
}

main();