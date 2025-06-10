#include <bits/stdc++.h>

using namespace std;
using pii = pair<int, int>;
namespace std {
  template <>
  struct hash<pii> {
    size_t operator()(const pii& p) const {
      return hash<int>()(p.first) ^ (hash<int>()(p.second) << 1);
    }
  };
}

class Graph {

public:
  bool ssmv = 0;
  int v; // 0..v-1
  vector<vector<int>> channels;
  vector<int> hosts;

  unordered_map<pii, vector<pii>> path;

  Graph() : v(0) {
      // 对于 vector 和 unordered_map，默认构造函数会自动将它们初始化为空
      // 所以这里不需要额外的代码去清空它们
  }

  void find_path();
  void VAR();
  void INIT();
  void ASSIGN();
  void SPEC();
};


int main (int argc, char** argv) {

  string output = "default.smv";
  bool ssmv = 0;

  // parse
  for (int i = 1; i < argc; ++i) {
    if (strcmp(argv[i], "-i") ) {
      freopen(argv[++i], "r", stdin);
    } else if (strcmp(argv[i], "-o")) {
      freopen(argv[++i], "w", stdout);
    } else if (strcmp(argv[i], "-ssmv")) {
      ssmv = 1;
    }
  }

  auto graph = Graph();
  int e, h;

  cin >> graph.v >> e >> h;
  graph.hosts.resize(h);
  graph.channels.resize(graph.v);

  int a, b;
  for (auto i = 0; i < e; ++i) {
    cin >> a >> b;
    graph.channels[a].push_back(b);
  }

  for (auto i = 0; i < h; ++i) {
    cin >> graph.hosts[i];
  }

  graph.ssmv = ssmv;
  graph.find_path();

  cout << "MODULE main\n";
  graph.VAR();
  graph.INIT();
  graph.ASSIGN();
  graph.SPEC();

  return 0;
}

void Graph::find_path() {
  for (auto i = 0; i < hosts.size(); ++i) {
    for (auto j = 0; j < hosts.size(); ++j) {
      if (i == j) continue;

      // 计算最短路径上的所有edge
      auto src = hosts[i], dst = hosts[j];
      vector<int> dist(v, INT_MAX);
      vector<int> prev(v, -1);
      priority_queue<pii, vector<pii>, std::greater<>> pq;
      dist[src] = 0;
      pq.emplace(0, src);
      
      while (not pq.empty()) {
        auto [cur_dist, u] = pq.top();
        pq.pop();

        if (u == dst) break;
        if (cur_dist > dist[u]) continue;

        for (auto v : channels[u]) {
          auto new_dist = dist[u] + 1;
          if (new_dist < dist[v]) {
            dist[v] = new_dist;
            prev[v] = u;
            pq.push({new_dist, v});
          }
        }

      }
      if (prev[dst] != -1) {
        std::vector<pii> edges;
        int current = dst;
        while (current != src) {
          int pred = prev[current];
          edges.emplace_back(pred, current);
          current = pred;
        }
        std::reverse(edges.begin(), edges.end());  // 反转得到正序路径
        path[{src, dst}] = edges; 
      }
      
    }
  }
}
void Graph::VAR() {

  cout << "\tVAR";
  for (auto i = 0; i < v; ++i) {
    for (auto to : channels[i]) {
      if (ssmv) {
  
      } else {
        printf("\t\tch%d_%d: 0..%d\n", i, to, (int)hosts.size());  
      }
    }
  }
  int cnt = 0;
  for (auto src = 0; src < hosts.size(); ++src ) {
    for (auto dst = 0; dst < hosts.size(); ++dst ) {

    } 
  }
  if (ssmv) {
    
  } else {

    printf("\t\tsignal: 0..%d\n", cnt);  
  }

}
void Graph::INIT() {

}
void Graph::ASSIGN() {

}
void Graph::SPEC() {

}