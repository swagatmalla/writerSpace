// React import for hooks like useState
import { useState } from "react";

// Import the pre-configured axios instance to call the backend API
import api from './api';

// Hardcoding a document ID for now
const DOC_ID  = 1; 

export default function AnalyzePage(){
    // Local state for the chosen  instruction
                                                    //default variable for the state var instruction
    const [instruction, setInstruction] = useState("summarize");

    // Local state for the LLM response
    const [response, setResponse] = useState("");

    // Local state to show loading spinner or disable button while request is in progress
    const[loading, setLoading] = useState(false);

    // Function triggered when the user clicks "Analyze"
    const runAnalyze  = async() => {
        setLoading(true); // show loading state
        setResponse(""); // clear previous response
    
        try {
            // Make POST request to backend API, sending the selected instruction
            const res = await api.post(`/documents/${DOC_ID}/analyze`, {
              instruction,   
            });


            // Set response from backend into state; fallback if missing
            setResponse(res.data.response ?? "(no response)");
        } catch(e:any){
            // If request fails, show the error message
            setResponse(e?.message ?? "Request failed");
        } finally {
            // Always turn off loading state when request completes
            setLoading(false);
        }
    };

    return(
        <div style = {{padding: 16, maxWidth: 720}}>
            <h1> Analyze Document </h1>
            {/* Dropdown to select instruction*/}
            <label>
                Instruction:&nbsp;
                <select
                    value = {instruction} // controlled component bound to state
                    onChange = {(e) => setInstruction(e.target.value)} //update state when user changes
                > 

                    <option value = "summarize">Summarize</option>
                    <option value = "analyze_tone"> Analyze tone</option>
                    <option value = "suggest_edits"> Suggest edit</option>
                </select>
            </label>
            {/* Button to run analysis */}
            <div style = {{marginTop: 12}}>
                <button onClick = {runAnalyze} disabled = {loading}>
                    {loading ? "Analyzing..." : "Analyze"}
                </button>
            </div>

            {/* Textarea to display the LLM response */}
            <div style = {{marginTop: 16}}>
                <h3>Response</h3>
                <textarea
                    value = {response} //controlled component bound to state
                    readOnly //user cannot edit
                    rows = {10}
                    style = {{width: "100%"}}
                />
            </div>

        </div>
    );

}