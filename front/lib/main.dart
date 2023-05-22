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

  @override
  String toString() {
    return '''
      {
        "id": $id,
        "title": "$title",
        "description": "$description",
        "date": "$date",
        "priority": "$priority",
        "duration": $duration,
        "status": "$status"
      }
    ''';
  }
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

  Future<void> updateCardData(int id, CardItem data) async {
    try {
      var url = Uri.http('localhost:3030', 'api/card/$id');
      var headers = {'Content-Type': 'application/json'};
      var response =
          await http.put(url, headers: headers, body: data.toString());

      if (response.statusCode == 200) {
        print('Card updated');
      } else {
        // Handle error response
        print('Request failed with status: ${response.statusCode}.');
      }
    } catch (error) {
      // Handle network or JSON parsing errors
      print('Error: $error');
    }
  }

  // Future<void> createCard(data) async {
  //   try {
  //     var url = Uri.http('localhost:3030', 'api/card');
  //     var response = await http.post(url, body: data);
  //     if (response.statusCode == 200) {
  //       print('Card created');
  //     } else {
  //       // Handle error response
  //       print('Request failed with status: ${response.statusCode}.');
  //     }
  //   } catch (error) {
  //     // Handle network or JSON parsing errors
  //     print('Error: $error');
  //   }
  // }

  Future<void> deleteCard(id) async {
    try {
      var url = Uri.http('localhost:3030', 'api/card/$id');
      var response = await http.delete(url);
      if (response.statusCode == 200) {
        print('Card deleted');
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
            style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 10),
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

                updateCardData(data.id, data);

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

  Future<void> createCard() async {
    // Show dialog to get input for new card
    CardItem? newCard = await showDialog<CardItem>(
      context: context,
      builder: (BuildContext context) {
        return NewCardDialog();
      },
    );

    // Create the new card if not null
    if (newCard != null) {
      try {
        var url = Uri.http('localhost:3030', 'api/card');
        var headers = {'Content-Type': 'application/json'};
        var response =
            await http.post(url, headers: headers, body: newCard.toString());

        if (response.statusCode == 200) {
          // Card created successfully, update the UI
          setState(() {
            cards.add(newCard);
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
        title: Text('Trello Clone Gaspard W'),
      ),
      body: Table(
        children: [
          TableRow(
            children: [
              buildColumn('To-do', todoCards),
              buildColumn('In Progress', inprogressCards),
              buildColumn('Completed', completedCards),
            ],
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: createCard,
        child: Icon(Icons.add),
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

class NewCardDialog extends StatefulWidget {
  @override
  _NewCardDialogState createState() => _NewCardDialogState();
}

class _NewCardDialogState extends State<NewCardDialog> {
  TextEditingController titleController = TextEditingController();
  TextEditingController descriptionController = TextEditingController();
  TextEditingController dateController = TextEditingController();
  TextEditingController priorityController = TextEditingController();
  TextEditingController durationController = TextEditingController();

  @override
  void dispose() {
    titleController.dispose();
    descriptionController.dispose();
    dateController.dispose();
    priorityController.dispose();
    durationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Create New Card'),
      content: SingleChildScrollView(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextField(
              controller: titleController,
              decoration: const InputDecoration(labelText: 'Title'),
            ),
            TextField(
              controller: descriptionController,
              decoration: const InputDecoration(labelText: 'Description'),
            ),
            TextField(
              controller: dateController,
              decoration: const InputDecoration(labelText: 'Date'),
            ),
            TextField(
              controller: priorityController,
              decoration: const InputDecoration(labelText: 'Priority'),
            ),
            TextField(
              controller: durationController,
              decoration: const InputDecoration(labelText: 'Duration'),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () {
            // Close the dialog and return null
            Navigator.of(context).pop();
          },
          child: const Text('Cancel'),
        ),
        ElevatedButton(
          onPressed: () {
            // Create a new CardItem and return it
            CardItem newCard = CardItem(
              id: DateTime.now().millisecondsSinceEpoch,
              title: titleController.text,
              description: descriptionController.text,
              date: dateController.text,
              priority: priorityController.text,
              duration: int.tryParse(durationController.text) ?? 0,
              status: 'To-do',
            );
            Navigator.of(context).pop(newCard);
          },
          child: const Text('Create'),
        ),
      ],
    );
  }
}
