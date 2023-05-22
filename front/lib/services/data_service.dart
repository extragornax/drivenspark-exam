import 'dart:convert';
import 'package:flutter/services.dart';
import 'package:http/http.dart' as http;
import '../widget/card.dart';

class DataItem {
  final int id;
  final String title;
  final String description;
  final String date;
  final String priority;
  final int duration;
  final String status;

  DataItem(this.id, this.title, this.description, this.date, this.priority,
      this.duration, this.status);

  DataItem.fromJson(Map<String, dynamic> json)
      : id = json['id'],
        title = json['title'],
        description = json['description'],
        date = json['date'],
        priority = json['priority'],
        duration = json['duration'],
        status = json['status'];

  Map<String, dynamic> toJson() => {
        'id': id,
        'title': title,
        'description': description,
        'date': date,
        'priority': priority,
        'duration': duration,
        'status': status,
      };
}

class DataService {
  static List<CardItem> toDos = [];
  static List<CardItem> inProgress = [];
  static List<CardItem> completed = [];

  static Future<void> loadData() async {
    // String jsonData = await rootBundle.loadString('assets/questions.json');
    // questions = json.decode(jsonData).cast<Map<String, dynamic>>();

    var url = Uri.http('localhost:3030', 'api/card');
    var response = await http.get(url);
    print("url: $url -> response: $response");

    if (response.statusCode == 200) {
      var jsonResponse = iterateJson(response.body);

      jsonResponse.forEach((el) {

        DataItem element = el;

        if (element.status == "todo") {
          toDos.add(CardItem(
              element.id,
              element.title,
              element.description,
              element.date,
              element.priority,
              element.duration,
              element.status));
        } else if element.status == "inprogress" {
          inProgress.add(CardItem(
              element.id,
              element.title,
              element.description,
              element.date,
              element.priority,
              element.duration,
              element.status));
        } else if element.status == "completed" {
          completed.add(CardItem(
              element.id,
              element.title,
              element.description,
              element.date,
              element.priority,
              element.duration,
              element.status));
        }
      })

      // print(jsonResponse);
    } else {
      print('Request failed with status: ${response.statusCode}.');
    }
  }

  static List<DataItem> iterateJson(String jsonStr) {
    List<dynamic> userMapItems = jsonDecode(jsonStr);

    List<DataItem> lst = [];

    userMapItems.forEach((element) {
      var parsed = DataItem.fromJson((element));
      lst.add(parsed);
    });

    print("myMap: $userMapItems t: $t");
    return lst;
  }
}
