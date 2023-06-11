import { Component, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";
import Note from "src/models/Note";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
  notes: Note[] = []
  content: string = ""

  ngOnInit(): void {
    this.fetchAllNotes();
  }

  public fetchNoteContent(fileName: string) {
    invoke<string>("fetch_note_content_from_cache", { fileName: fileName }).then((content: string) => {
      this.content = content
    })
  }

  private fetchAllNotes() {
    invoke<string>("fetch_all_notes_from_cache", {}).then((res: any) => {
      this.notes = this.mapToType(res)
      console.log(this.notes)
    })
  }

  private mapToType(result: any[]): Note[] {
    let notes: Note[] = []
    result.forEach(res => {
      let d = new Date(0)
      console.log(res.creation_time)
      d.setUTCSeconds(res.creation_time.secs_since_epoch)
      notes.push({ fileName: res.file_name, path: res.path, text: res.text, creation_time: d })
    })

    return notes
  }
}
