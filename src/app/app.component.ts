import { Component, OnInit } from "@angular/core";
import Note from "src/models/Note";
import { NoteService } from "./services/note.service";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  notes: Note[] = []
  selectedNote: string = ""

  constructor(private noteSerivce: NoteService) { }

  async ngOnInit(): Promise<void> {
    await this.fetchAllNotes()
  }

  private async fetchAllNotes() {
    this.notes = await this.noteSerivce.fetchNotes()
  }
}
