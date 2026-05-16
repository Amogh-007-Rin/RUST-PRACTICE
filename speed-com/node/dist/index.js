function main() {
    let count = 0;
    const startTime = performance.now();
    for (let i = 0; i < 1000000001; i++) {
        count = count + i;
    }
    const endTime = performance.now();
    console.log(count);
    const totalTime = endTime - startTime;
    console.log(`Time Taken ${totalTime.toFixed(4)} milliseconds`);
}
main();
export {};
//# sourceMappingURL=index.js.map