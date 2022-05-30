#include <iostream>
#include <stdlib.h>
#include <math.h>
#include <fstream>
#include <time.h>


using namespace std;

int main(int argc, char **argv){

    if(argc != 3){
        cout << "You need to give number of nodes and rewiring probability(0-100)\n";
        return 0;
    }

    ofstream MyFile("random_graph.txt");
    ofstream TempFile("edges_to_change.txt"); // gia na dhmiourgei to adeio arxeio
    time_t t;
    srand((unsigned) time(&t));

    int nofnodes = atoi(argv[1]); // number of nodes for graph generation
    double prob = (double) atoi(argv[2]) / 100; // rewiring probability

    clock_t tim;
    tim = clock();

    for(int i = 0; i<nofnodes-1;i++){
        if(i % 10000 == 0){
            cout<<i;
            cout<<"\n";
        }
        for(int j=i+1;j<nofnodes;j++){
            int temp = rand() % 100;
            if(temp < (int)(prob*100))
                MyFile << to_string(i) + " " + to_string(j) + "\n";
            
        }

    }

    tim = clock() - tim;
    cout << ((double)tim)/CLOCKS_PER_SEC << " seconds\n";

    MyFile.close();
    TempFile.close();

    return 0;
}