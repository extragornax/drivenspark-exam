# Drivenspark Exam - Card Manager

This card Manager is a simple Flutter application that allows you to manage a list of cards. Each card represents a task or an item with a title, description, date, status and priority. You can perform various operations such as creating new cards, viewing existing cards, editing existing cards, and deleting cards.

## Features

- View a list of cards with their titles and descriptions.
- Edit the title and description of a card.
- Delete a card from the list.
- Create a new card with a title and description.

## Getting Started

To run the Card Manager application, follow these steps:

1. Ensure that you have Flutter installed on your machine. For installation instructions, refer to the official [Flutter website](https://flutter.dev/docs/get-started/install).

2. Clone this repository or download the source code.

3. Open the project in your preferred Flutter development environment (VS Code is the best).

4. Run the following command in the terminal to start the application:x
  
  ```bash
  flutter run
  ```

5. Select the option 1 (linux) or 2 (chrome) to run the application.

6. To reload the app while it's running, press "r". To quit, press "q".


## Usage

Upon launching the Card Manager application, you'll see a list of cards displayed on the home screen. Each card represents a task or an item.

- To edit a card, tap the "Edit" button next to the card. A dialog will appear, allowing you to update the title, description, status and priority.

- To delete a card, tap the "Delete" button next to the card. A confirmation dialog will appear, and upon confirmation, the card will be deleted from the list.

- To create a new card, tap the floating action button (the "+" icon) at the bottom right corner of the screen. A dialog will appear, prompting you to enter the title, description, status and priority for the new card. Once you provide the required information and tap "Create," the new card will be added to the list.

## Customization

You can customize the Card Manager application by making changes to the codebase:

- Modify the theme: Open `main.dart` and update the `theme` property in the `MaterialApp` widget to change the primary color or other theme attributes.

- Adjust card layout: Open `main.dart` and modify the `Card` widget and its child `ListTile` to customize the appearance and content of each card.

- Enhance functionality: Expand the capabilities of the application by adding new features such as due dates, labels, or search functionality. Modify the `CardItem` class and related methods in `main.dart` to support the desired functionality.

- Splitting the network parts from the code, and setting the card handler in it's own module would be ideal for larger projects and cleaner code.

## Contributions

Contributions to the Card Manager application are welcome! If you find any issues or have ideas for enhancements, feel free to open an issue or submit a pull request on the GitHub repository.

## License

The Card Manager application is open-source and released under the [MIT License](LICENSE). You are free to use, modify, and distribute the code as per the terms of the license.

## Credits

The Card Manager application was developed by Gaspard W. (Extragornax) as a demonstration of Flutter app development for the drivenspark exam.
