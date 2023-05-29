import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  greetingMessage = "";

  ngOnInit(): void {
    invoke<string>("write_to_file", { text: "this is a test", fileName: "test1.txt" }).then(() => {
      console.log("done")
    })
  }

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });


  }
}
