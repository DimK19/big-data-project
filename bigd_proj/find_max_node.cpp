#include <iostream>
#include <stdlib.h>
#include <math.h>
#include <fstream>
#include <time.h>


using namespace std;

int main(){

    ifstream TempInt("random_graph.txt");

    int a,b;
    int max = 0;
    
    while (TempInt >> a >> b){
            if(a>max)
                max =a;
            if(b>max)
                max = b;
        }

    cout << max << "\n";
    
    TempInt.close();    

    return 0;
}