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
  

  void find_path();
  void VAR();
  void INIT();
  void WRITE();
private:
  int signal_cnt;
};


int main (int argc, char** argv) {

  string output = "default.smv";
  bool ssmv = 0;

  // parse
  for (int i = 1; i < argc; ++i) {
    if (strcmp(argv[i], "-i") == 0 ) {
      freopen(argv[++i], "r", stdin);
    } else if (strcmp(argv[i], "-o") == 0) {
      freopen(argv[++i], "w", stdout);
    } else if (strcmp(argv[i], "-ssmv") == 0) {
      ssmv = 1;
    }
  }

  auto graph = Graph();
  int e = 0, h = 0;

  scanf("%d %d %d\n", &graph.v, &e, &h);
  graph.hosts.resize(h);
  graph.channels.resize(graph.v);

  int a, b;
  for (auto i = 0; i < e; ++i) {
    scanf("%d %d\n", &a, &b);
    graph.channels[a-1].push_back(b-1);
  }

  for (auto i = 0; i < h; ++i) {
    scanf("%d", &graph.hosts[i]);
    --graph.hosts[i];
  }

  graph.ssmv = ssmv;
  graph.find_path();

  cout << "MODULE main\n";
  graph.WRITE();

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
        std::reverse(edges.begin(), edges.end());
        path[{i, j}] = std::move(edges);
      }
      
    }
  }
}
void Graph::VAR() {

  cout << "  VAR\n";
  for (auto i = 0; i < v; ++i) {
    for (auto to : channels[i]) {
      if (ssmv) {
  
      } else {
        printf("    ch%d_%d: 0..%d;\n", i + 1, to + 1, (int)hosts.size());  
      }
    }
  }
  if (ssmv) {
    
  } else {

    printf("    signal: 0..%d;\n", signal_cnt);  
  }

}
void Graph::INIT() {
  cout << "  INIT\n";
  for (int i = 0; i < v; ++i) {
    for (int j = 0; j < channels[i].size(); ++j) {
      int to = channels[i][j];
      if (!ssmv) {
        printf("    ch%d_%d = 0", i + 1, to + 1);
        if (i == v - 1 && j == channels[i].size() - 1) {
          printf(";\n");
        } else {
          printf(" &\n");
        }
      }
    }
  }
}

void Graph::WRITE() {

  // TODO: 先重构数据结构，再输出，这样会省很多事
  unordered_map<int, set<pair<pii, bool>>> dst_paths;
    
  for (const auto& path_entry : path) {
    const auto& src_dst = path_entry.first;
    const auto& edges = path_entry.second;
    int src = src_dst.first;
    int dst = src_dst.second;
    
    for (const pii& edge : edges) {
      bool issue = (edge.first == src);
      
      if ( issue && dst_paths[dst].find({edge, false}) != dst_paths[dst].end() ) {
        dst_paths[dst].erase({edge, 0});
        dst_paths[dst].emplace(edge, 1);
      } else if ( issue && dst_paths[dst].find({edge, false}) == dst_paths[dst].end() ) {
        dst_paths[dst].emplace(edge, 1);
      } else if ( not issue && dst_paths[dst].find({edge, true}) == dst_paths[dst].end() ) {
        dst_paths[dst].emplace(edge, 0);
      }
    }
  }

  // for (int i = 0; i < hosts.size(); ++i) {
  //   cout << i << "\n";
  //   for (const auto& [edge, issue] : dst_paths[i]) {
  //     printf("%d %d : %d \n", edge.first, edge.second, issue);
  //   }
  // }

  using tisi = tuple<int, string, int>;
  std::unordered_map<string, vector<tisi>> assign_cases;

  int signal = 0;
  for (auto dst : hosts) {
    for (auto& [edge, issue] : dst_paths[dst]) {
      string ch = "ch" + to_string(edge.first + 1) + "_" + to_string(edge.second + 1);
      if (issue) {
        string rule = ch + " = 0";
        assign_cases[ch].push_back({signal, rule, dst + 1 });
        ++signal;
      } 

      {
        string rule = ch + " = " + to_string(dst + 1);
        for (auto& [ e, _] : dst_paths[dst] ) {
          if (edge.second == e.first) {
            string next_ch = "ch" + to_string(e.first + 1) + "_" + to_string(e.second + 1);
            string temp_rule = rule + " & " + next_ch + " = 0";
            assign_cases[ch].push_back({signal, temp_rule, 0 });
            assign_cases[next_ch].push_back({signal, temp_rule, dst + 1 });
            ++signal;
            break;
          }
        }
      }

      if (edge.second == dst) {
        string rule = ch + " = " + to_string(dst + 1);
        assign_cases[ch].push_back({signal, rule, 0});
        ++signal;
      }
    }
  }
  signal_cnt = signal - 1;
  VAR();
  INIT();
  cout << "  ASSIGN\n";

  if (!ssmv) {
    printf("    init(signal) := 0..%d;\n", signal_cnt);
    printf("    next(signal) := 0..%d;\n\n", signal_cnt);
    for (auto& [ch, cases] : assign_cases) {
      printf("    next(%s) := \n", ch.c_str());
      printf("      case\n");
      for (auto& [ fire, rule, target] : cases) {
        printf("        signal = %d & %s : %d;\n", fire, rule.c_str(), target);
      }
      printf("        TRUE : %s;\n", ch.c_str());
      printf("      esac;\n\n");
    }
  }


  // SPEC
  if (!ssmv) {
    cout << "  CTLSPEC !EF!(";
    bool start = 1;
    set<string> ss;
    for (auto& [ch, cases] : assign_cases) {
      for (auto& [ _, rule, _] : cases) {
        if (ss.find(rule) != ss.end()) continue;

        if (start) {
          printf(" \n");
          start = 0;
        } else {
          printf(" | \n");
        }
        
        printf("    ( %s )", rule.c_str());
        ss.emplace(rule);
        
      }
    }
    cout << "\n  )\n";
  }

}
