### **1. High-Level Overview**

This diagram shows the overall structure of the application, including the main
entry point, command processing, and execution flow.

```mermaid
graph LR
    subgraph main
        A[Start] --> B[Parse command-line arguments]
        B --> C[Generate entry paths]
        C --> D[Process entries]
        D --> E[Generate summaries]
        E --> F[Output results]
        F --> G[End]
    end

    subgraph Process entries
        subgraph Entry processing
            H[Filter and process entries] --> I[Generate file paths]
        end
        subgraph Parallel processing
            J[Spawn tasks] --> K[Generate summaries]
            K --> L[Collect results]
        end
        subgraph Sequential processing
            M[Process entries one by one] --> N[Generate summaries]
        end
    end

    subgraph Generate summaries
        O[Retrieve commits] --> P[Generate diffs]
        P --> Q[Insert into DashMap]
    end
```

---

### **2. Main Function Flow**

This diagram shows the control flow within the `main` function, including the
initialization of the command structure and the execution of the async function.

```mermaid
graph LR
    subgraph main
        A[Start] --> B[Create command structure]
        B --> C[Parse command-line arguments]
        C --> D[Generate entry paths]
        D --> E[Process entries]
        E --> F[Generate summaries]
        F --> G[Output results]
        G --> H[End]
    end
```

---

### **3. Command Processing Flow**

This diagram shows the flow of processing command-line arguments and generating
entry paths.

```mermaid
graph LR
    subgraph Command processing
        A[Parse arguments] --> B[Generate entry paths]
        B --> C[Filter entries]
        C --> D[Process entries]
    end

    subgraph Generate entry paths
        E[Walk directory] --> F[Filter by exclude patterns]
        F --> G[Generate file paths]
    end
```

---

### **4. Parallel Processing Flow**

This diagram shows the flow of processing entries in parallel, including task
spawning and result collection.

```mermaid
graph LR
    subgraph Parallel processing
        A[Spawn tasks] --> B[Generate summaries]
        B --> C[Collect results]
        C --> D[Output results]
    end

    subgraph Generate summaries
        E[Retrieve commits] --> F[Generate diffs]
        F --> G[Insert into DashMap]
    end
```

---

### **5. Sequential Processing Flow**

This diagram shows the flow of processing entries sequentially, including
generating summaries and outputting results.

```mermaid
graph LR
    subgraph Sequential processing
        A[Process entries one by one] --> B[Generate summaries]
        B --> C[Output results]
    end

    subgraph Generate summaries
        D[Retrieve commits] --> E[Generate diffs]
        E --> F[Insert into DashMap]
    end
```

---

### **6. Summary Generation Flow**

This diagram shows the flow of generating summaries, including retrieving
commits, generating diffs, and inserting results into a DashMap.

```mermaid
graph LR
    subgraph Summary generation
        A[Retrieve commits] --> B[Generate diffs]
        B --> C[Insert into DashMap]
        C --> D[Output results]
    end
```

---

### **7. Function Call Graph**

This diagram shows the relationships between major functions and their calls.

```mermaid
graph LR
    main --> parse_arguments
    parse_arguments --> generate_entry_paths
    generate_entry_paths --> filter_entries
    filter_entries --> process_entries
    process_entries --> generate_summaries
    generate_summaries --> output_results
```

---

### **8. Concurrency Flow**

This diagram shows the concurrency model used in the parallel processing of
entries.

```mermaid
graph LR
    main --> spawn_tasks
    spawn_tasks --> generate_summaries
    generate_summaries --> collect_results
    collect_results --> output_results
```

---

### **9. Error Handling Flow**

This diagram shows the error handling flow within the application.

```mermaid
graph LR
    main --> handle_errors
    handle_errors --> log_errors
    log_errors --> exit
```

---

### **10. Data Flow**

This diagram shows the data flow through the application, from input to output.

```mermaid
graph LR
    input --> parse_arguments
    parse_arguments --> generate_entry_paths
    generate_entry_paths --> process_entries
    process_entries --> generate_summaries
    generate_summaries --> output_results
```
