import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:intl/intl.dart';

void main() {
  runApp(MyApp());
}

class CardItem {
  final int id;
  String title;
  String description;
  String date;
  String priority;
  int duration;
  String status;

  CardItem({
    required this.id,
    required this.title,
    required this.description,
    required this.date,
    required this.priority,
    required this.duration,
    required this.status,
  });
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Trello Clone Gaspard W',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: MyHomePage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  List<CardItem> cards = [];

  @override
  void initState() {
    super.initState();
    fetchData();
  }

  Future<void> fetchData() async {
    try {
      var url = Uri.http('localhost:3030', 'api/card');
      var response = await http.get(url);
      if (response.statusCode == 200) {
        var data = jsonDecode(response.body) as List<dynamic>;
        List<CardItem> fetchedCards = data.map((item) {
          return CardItem(
            id: item['id'],
            title: item['title'],
            description: item['description'],
            date: item['date'],
            priority: item['priority'],
            duration: item['duration'],
            status: item['status'],
          );
        }).toList();

        setState(() {
          cards = fetchedCards;
        });
      } else {
        // Handle error response
        print('Request failed with status: ${response.statusCode}.');
      }
    } catch (error) {
      // Handle network or JSON parsing errors
      print('Error: $error');
    }
  }

  Widget buildColumn(String columnTitle, List<CardItem> columnCards) {
    return TableCell(
      child: Column(
        children: [
          Text(
            columnTitle,
            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          SizedBox(height: 10),
          for (CardItem card in columnCards)
            Draggable(
              data: card,
              feedback: buildCardWidget(card),
              childWhenDragging: Container(),
              child: buildCardWidget(card),
            ),
          DragTarget<CardItem>(
            builder: (
              BuildContext context,
              List<CardItem?> candidateData,
              List<dynamic> rejectedData,
            ) {
              return Container(
                height: 200,
                width: double.infinity,
                color: Colors.grey.shade200,
              );
            },
            onAccept: (CardItem data) {
              setState(() {
                // Remove the card from its previous column
                cards.remove(data);

                // Update the status of the card based on the column it was dropped into
                if (columnTitle == 'To-do') {
                  data.status = 'todo';
                } else if (columnTitle == 'In Progress') {
                  data.status = 'inprogress';
                } else if (columnTitle == 'Completed') {
                  data.status = 'completed';
                }

                print("columnTitle: $columnTitle, data.status: ${data.status}");

                // Add the card to the new column
                cards.add(data);
              });
            },
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    List<CardItem> todoCards =
        cards.where((card) => card.status == 'todo').toList();
    List<CardItem> inprogressCards =
        cards.where((card) => card.status == 'inprogress').toList();
    List<CardItem> completedCards =
        cards.where((card) => card.status == 'completed').toList();

    return Scaffold(
      appBar: AppBar(
        title: const Text('Trello Clone Gaspard W'),
      ),
      body: Table(
        children: [
          TableRow(
            children: [
              TableCell(
                child: Column(
                  children: [
                    const Text(
                      'To-do',
                      style:
                          TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 10),
                    for (CardItem card in todoCards)
                      Draggable(
                        data: card,
                        feedback: buildCardWidget(card),
                        childWhenDragging: Container(),
                        child: buildCardWidget(card),
                      ),
                  ],
                ),
              ),
              TableCell(
                child: Column(
                  children: [
                    const Text(
                      'In Progress',
                      style:
                          TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 10),
                    for (CardItem card in inprogressCards)
                      Draggable(
                        data: card,
                        feedback: buildCardWidget(card),
                        childWhenDragging: Container(),
                        child: buildCardWidget(card),
                      ),
                  ],
                ),
              ),
              TableCell(
                child: Column(
                  children: [
                    const Text(
                      'Completed',
                      style:
                          TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 10),
                    for (CardItem card in completedCards)
                      Draggable(
                        data: card,
                        feedback: buildCardWidget(card),
                        childWhenDragging: Container(),
                        child: buildCardWidget(card),
                      ),
                  ],
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  BoxDecoration myBoxDecoration() {
    return BoxDecoration(border: Border.all(), color: Colors.blueAccent);
  }

  Widget buildCardWidget(CardItem card) {
    // final formattedDate =
    //     DateFormat('dd-MM-yyyy').format(DateTime.parse(card.date));

    final formattedDate = DateFormat().format(DateTime.parse(card.date));

    return Container(
      padding: const EdgeInsets.all(10),
      // color: Colors.grey,
      decoration: myBoxDecoration(),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Title: ${card.title}'),
          Text('Description: ${card.description}'),
          Text('Date: $formattedDate'),
          Text('Priority: ${card.priority}'),
          Text('Duration: ${card.duration.toString()}'),
          Text('Status: ${card.status}'),
        ],
      ),
    );
  }
}
