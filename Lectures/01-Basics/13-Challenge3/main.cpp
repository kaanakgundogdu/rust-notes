#include <iostream>
#include <vector>

int main()
{
    std::vector <int> vector1;
    std::vector <int> vector2;
    
    vector1.push_back(10);
    vector1.push_back(20);
    std::cout<<vector1.at(0)<<std::endl;
    std::cout<<vector1.at(1)<<std::endl;
    std::cout<<"Size: "<<vector1.size()<<std::endl;
    
    
    vector2.push_back(100);
    vector2.push_back(200);
    std::cout<<vector2.at(0)<<std::endl;
    std::cout<<vector2.at(1)<<std::endl;
    std::cout<<"Size: "<<vector2.size()<<std::endl;
    
    std::vector <std::vector<int>> vector_2d;
    
    std::cout<<"\nNew 2d vector"<<std::endl;

    vector_2d.push_back(vector1);
    vector_2d.push_back(vector2);
    std::cout<<vector_2d.at(0).at(0)<<" "<<vector_2d.at(0).at(1) <<std::endl;
    std::cout<<vector_2d.at(1).at(0)<<" "<<vector_2d.at(1).at(1)<<std::endl;
    std::cout<<"Size: "<<vector_2d.size()<<std::endl;
    
    vector1.at(0)=1000;
    std::cout<<"\nUpdated 2d vector"<<std::endl;

    std::cout<<vector_2d.at(0).at(0)<<" "<<vector_2d.at(0).at(1) <<std::endl;
    std::cout<<vector_2d.at(1).at(0)<<" "<<vector_2d.at(1).at(1)<<std::endl;
    std::cout<<"Size: "<<vector_2d.size()<<std::endl;
    
    std::cout<<"\nUpdated vector1"<<std::endl;

    std::cout<<vector1.at(0)<<std::endl;
    std::cout<<vector1.at(1)<<std::endl;
    std::cout<<"Size: "<<vector1.size()<<std::endl;
    
	return 0;
}
