// React import for hooks like useState
import { use, useEffect, useState} from "react";
import {useParams} from 'react-router-dom';

// Import the pre-configured axios instance to call the backend API
import api from '../api';

export default function AnalyzePage(){
    const {docId} = useParams();
    // Local state for the chosen  instruction
                                                    //default variable for the state var instruction
    const [instruction, setInstruction] = useState("summarize");

    // Local state for the LLM response
    const [response, setResponse] = useState("");

    // Local state to show loading spinner or disable button while request is in progress
    const[loading, setLoading] = useState(false);

    // states for custom instruction
    const [useCustomInstruction, setUseCustomInstruction] = useState(false);
    const [customInstruction, setCustomInstruction] = useState("");


    interface Document{
        id: number;
        project_id: number | null;
        title: string;
        content: string | null;
        media_type: string | null;
        file_path: string | null; 
        created_at: string |  null;
        updated_at: string | null;

        
    }
    // Local state for the fetched document
    const [document, setDoc] = useState<Document | null>(null);


    // do this when the page loads
    useEffect(() =>{
        const fetchDocument = async() => {
            try{
                const res = await api.get(`/documents/${docId}`);
                setDoc(res.data);

            }catch(error){
                console.error('Failed to fetch document:', error);
            }
        };

        if(docId){
            fetchDocument();
        }

    }, [docId]); // dependancy array [docId] controls WHEN this runs; [] ~ run only once on mount


    // Function triggered when the user clicks "Analyze"
    const runAnalyze  = async() => {
        setLoading(true); // show loading state
        setResponse(""); // clear previous response
        setInstruction(useCustomInstruction? customInstruction: instruction);
        try {
            // Make POST request to backend API, sending the selected instruction
            const res = await api.post(`/documents/${docId}/analyze`, {
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
            <h1 className = "poiret-one-regular"> {document?.title || "Analyze Document"} </h1>
            <div>
                {document?.content || "Content not found"};
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

            <br />
            <div>
                <label>
                    <input
                        type = "checkbox"
                        checked = {useCustomInstruction}
                        onChange = {(e) => setUseCustomInstruction(e.target.checked)}
                    />
                    Custom Instruction
                </label>
            </div>

            
                {!useCustomInstruction ? (
                    <label>

                    <br />
                    Instruction:&nbsp;
                    <select
                        value = {instruction} // controlled component bound to state
                        onChange = {(e) => setInstruction(e.target.value)} //update state when user changes
                    > 

                        <option value = "summarize">Summarize</option>
                        <option value = "analyze_tone"> Analyze tone</option>
                        <option value = "suggest_edits"> Suggest edit</option>
                    </select>
                    </label>)
                :(
                    <label>
                        Custom instruction:&nbsp;
                        <input
                            type = "text"
                            value = {customInstruction}
                            onChange={(e) => setCustomInstruction(e.target.value)}
                            placeholder="Enter your custom instruction..."
                        />
                    </label>
                )}
            {/* Button to run analysis */}
            <div style = {{marginTop: 12}}>
                <button onClick = {runAnalyze} disabled = {loading}>
                    {loading ? "Analyzing..." : "Analyze"}
                </button>
            </div>

        </div>
    );

}