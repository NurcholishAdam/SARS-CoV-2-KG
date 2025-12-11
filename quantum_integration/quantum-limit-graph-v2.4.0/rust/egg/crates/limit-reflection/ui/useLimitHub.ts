// useLimitHub.ts
// React hook for LIMIT Hub + Reflection integration
// Provides evidence + reflection side-by-side visualization

import { useState, useEffect, useCallback } from 'react';

// ============================================================================
// Types
// ============================================================================

export interface EvidenceItem {
  id: string;
  content: string;
  confidence: number;
  provenance: string[];
}

export interface ReflectionSummary {
  steps_count: number;
  final_confidence: number;
  insights: MetaCognitiveInsights;
}

export interface MetaCognitiveInsights {
  total_steps: number;
  average_confidence: number;
  total_errors: number;
  unique_error_types: number;
  suggestions_count: number;
}

export interface ReflectWithEvidenceResponse {
  query: string;
  evidence: EvidenceItem[];
  reflection: ReflectionSummary;
  combined_confidence: number;
}

export interface Submission {
  id: string;
  content: string;
  confidence: number;
  provenance: string[];
  quality_score: number;
  metadata: Record<string, string>;
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
  requires_review: boolean;
}

// ============================================================================
// Hook Configuration
// ============================================================================

interface UseLimitHubConfig {
  baseUrl?: string;
  autoRefresh?: boolean;
  refreshInterval?: number;
}

const DEFAULT_CONFIG: UseLimitHubConfig = {
  baseUrl: 'http://localhost:3002',
  autoRefresh: false,
  refreshInterval: 5000,
};

// ============================================================================
// Main Hook
// ============================================================================

export function useLimitHub(config: UseLimitHubConfig = {}) {
  const finalConfig = { ...DEFAULT_CONFIG, ...config };
  const { baseUrl, autoRefresh, refreshInterval } = finalConfig;

  // State
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [evidenceData, setEvidenceData] = useState<ReflectWithEvidenceResponse | null>(null);
  const [submissions, setSubmissions] = useState<Submission[]>([]);
  const [insights, setInsights] = useState<MetaCognitiveInsights | null>(null);

  // ============================================================================
  // API Calls
  // ============================================================================

  const reflectWithEvidence = useCallback(
    async (query: string): Promise<ReflectWithEvidenceResponse | null> => {
      setLoading(true);
      setError(null);

      try {
        const response = await fetch(`${baseUrl}/reflect-with-evidence`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ query }),
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data: ReflectWithEvidenceResponse = await response.json();
        setEvidenceData(data);
        return data;
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Unknown error';
        setError(errorMessage);
        return null;
      } finally {
        setLoading(false);
      }
    },
    [baseUrl]
  );

  const submitData = useCallback(
    async (submission: Submission): Promise<boolean> => {
      setLoading(true);
      setError(null);

      try {
        const response = await fetch(`${baseUrl}/submit`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(submission),
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        return true;
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Unknown error';
        setError(errorMessage);
        return false;
      } finally {
        setLoading(false);
      }
    },
    [baseUrl]
  );

  const fetchSubmissions = useCallback(async () => {
    try {
      const response = await fetch(`${baseUrl}/submissions`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data = await response.json();
      setSubmissions(data);
    } catch (err) {
      console.error('Failed to fetch submissions:', err);
    }
  }, [baseUrl]);

  const fetchInsights = useCallback(async () => {
    try {
      const response = await fetch(`${baseUrl}/insights`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data: MetaCognitiveInsights = await response.json();
      setInsights(data);
    } catch (err) {
      console.error('Failed to fetch insights:', err);
    }
  }, [baseUrl]);

  const validateSubmission = useCallback(
    async (submission: Submission): Promise<ValidationResult | null> => {
      try {
        const response = await fetch(`${baseUrl}/validate`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(submission),
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        return await response.json();
      } catch (err) {
        console.error('Failed to validate submission:', err);
        return null;
      }
    },
    [baseUrl]
  );

  // ============================================================================
  // Auto-refresh Effect
  // ============================================================================

  useEffect(() => {
    if (autoRefresh) {
      const interval = setInterval(() => {
        fetchSubmissions();
        fetchInsights();
      }, refreshInterval);

      return () => clearInterval(interval);
    }
  }, [autoRefresh, refreshInterval, fetchSubmissions, fetchInsights]);

  // ============================================================================
  // Return Hook Interface
  // ============================================================================

  return {
    // State
    loading,
    error,
    evidenceData,
    submissions,
    insights,

    // Actions
    reflectWithEvidence,
    submitData,
    fetchSubmissions,
    fetchInsights,
    validateSubmission,

    // Utilities
    clearError: () => setError(null),
    clearEvidenceData: () => setEvidenceData(null),
  };
}

// ============================================================================
// React Component Example
// ============================================================================

export interface LimitHubVisualizerProps {
  query: string;
  onQueryChange?: (query: string) => void;
}

export function LimitHubVisualizer({ query, onQueryChange }: LimitHubVisualizerProps) {
  const {
    loading,
    error,
    evidenceData,
    insights,
    reflectWithEvidence,
    fetchInsights,
  } = useLimitHub({ autoRefresh: true });

  useEffect(() => {
    fetchInsights();
  }, [fetchInsights]);

  const handleReflect = async () => {
    if (query.trim()) {
      await reflectWithEvidence(query);
    }
  };

  return (
    <div className="limit-hub-visualizer">
      {/* Query Input */}
      <div className="query-section">
        <input
          type="text"
          value={query}
          onChange={(e) => onQueryChange?.(e.target.value)}
          placeholder="Enter your query..."
          className="query-input"
        />
        <button onClick={handleReflect} disabled={loading} className="reflect-button">
          {loading ? 'Reflecting...' : 'Reflect with Evidence'}
        </button>
      </div>

      {/* Error Display */}
      {error && (
        <div className="error-message">
          <strong>Error:</strong> {error}
        </div>
      )}

      {/* Side-by-Side Layout */}
      {evidenceData && (
        <div className="evidence-reflection-container">
          {/* Left: Evidence Panel */}
          <div className="evidence-panel">
            <h3>Evidence ({evidenceData.evidence.length} items)</h3>
            {evidenceData.evidence.map((item) => (
              <div key={item.id} className="evidence-item">
                <div className="evidence-header">
                  <span className="evidence-id">{item.id}</span>
                  <span className="confidence-badge">
                    {(item.confidence * 100).toFixed(0)}%
                  </span>
                </div>
                <p className="evidence-content">{item.content}</p>
                <div className="provenance">
                  <strong>Sources:</strong> {item.provenance.join(', ')}
                </div>
              </div>
            ))}
          </div>

          {/* Right: Reflection Panel */}
          <div className="reflection-panel">
            <h3>Meta-Cognitive Reflection</h3>
            
            <div className="reflection-summary">
              <div className="metric">
                <label>Steps:</label>
                <span>{evidenceData.reflection.steps_count}</span>
              </div>
              <div className="metric">
                <label>Confidence:</label>
                <span>{(evidenceData.reflection.final_confidence * 100).toFixed(1)}%</span>
              </div>
              <div className="metric">
                <label>Combined:</label>
                <span className="combined-confidence">
                  {(evidenceData.combined_confidence * 100).toFixed(1)}%
                </span>
              </div>
            </div>

            <div className="insights-section">
              <h4>Insights</h4>
              <div className="insights-grid">
                <div className="insight-item">
                  <label>Total Steps:</label>
                  <span>{evidenceData.reflection.insights.total_steps}</span>
                </div>
                <div className="insight-item">
                  <label>Avg Confidence:</label>
                  <span>
                    {(evidenceData.reflection.insights.average_confidence * 100).toFixed(1)}%
                  </span>
                </div>
                <div className="insight-item">
                  <label>Errors:</label>
                  <span>{evidenceData.reflection.insights.total_errors}</span>
                </div>
                <div className="insight-item">
                  <label>Suggestions:</label>
                  <span>{evidenceData.reflection.insights.suggestions_count}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Global Insights */}
      {insights && (
        <div className="global-insights">
          <h3>System Insights</h3>
          <div className="insights-summary">
            <span>Total Steps: {insights.total_steps}</span>
            <span>Avg Confidence: {(insights.average_confidence * 100).toFixed(1)}%</span>
            <span>Errors: {insights.total_errors}</span>
          </div>
        </div>
      )}
    </div>
  );
}

// ============================================================================
// CSS Styles (to be added to your stylesheet)
// ============================================================================

export const LIMIT_HUB_STYLES = `
.limit-hub-visualizer {
  padding: 20px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.query-section {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.query-input {
  flex: 1;
  padding: 12px;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-size: 16px;
}

.reflect-button {
  padding: 12px 24px;
  background: #2563eb;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
}

.reflect-button:hover {
  background: #1d4ed8;
}

.reflect-button:disabled {
  background: #9ca3af;
  cursor: not-allowed;
}

.error-message {
  padding: 12px;
  background: #fee2e2;
  border: 1px solid #ef4444;
  border-radius: 8px;
  color: #991b1b;
  margin-bottom: 20px;
}

.evidence-reflection-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  margin-top: 20px;
}

.evidence-panel, .reflection-panel {
  border: 2px solid #e0e0e0;
  border-radius: 12px;
  padding: 20px;
  background: white;
}

.evidence-panel h3, .reflection-panel h3 {
  margin-top: 0;
  color: #1f2937;
  border-bottom: 2px solid #e0e0e0;
  padding-bottom: 10px;
}

.evidence-item {
  padding: 15px;
  background: #f9fafb;
  border-radius: 8px;
  margin-bottom: 15px;
  border-left: 4px solid #3b82f6;
}

.evidence-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 10px;
}

.evidence-id {
  font-weight: 600;
  color: #4b5563;
  font-size: 14px;
}

.confidence-badge {
  background: #10b981;
  color: white;
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
}

.evidence-content {
  margin: 10px 0;
  color: #374151;
  line-height: 1.6;
}

.provenance {
  font-size: 12px;
  color: #6b7280;
  margin-top: 10px;
}

.reflection-summary {
  display: flex;
  gap: 20px;
  margin-bottom: 20px;
  padding: 15px;
  background: #f0f9ff;
  border-radius: 8px;
}

.metric {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.metric label {
  font-size: 12px;
  color: #6b7280;
  font-weight: 600;
}

.metric span {
  font-size: 20px;
  font-weight: 700;
  color: #1f2937;
}

.combined-confidence {
  color: #2563eb !important;
}

.insights-section {
  margin-top: 20px;
}

.insights-section h4 {
  margin-bottom: 15px;
  color: #374151;
}

.insights-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 15px;
}

.insight-item {
  padding: 12px;
  background: #f9fafb;
  border-radius: 6px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.insight-item label {
  font-size: 14px;
  color: #6b7280;
  font-weight: 500;
}

.insight-item span {
  font-size: 16px;
  font-weight: 700;
  color: #1f2937;
}

.global-insights {
  margin-top: 20px;
  padding: 20px;
  background: #fef3c7;
  border-radius: 12px;
  border: 2px solid #f59e0b;
}

.global-insights h3 {
  margin-top: 0;
  color: #92400e;
}

.insights-summary {
  display: flex;
  gap: 30px;
  font-weight: 600;
  color: #78350f;
}

@media (max-width: 768px) {
  .evidence-reflection-container {
    grid-template-columns: 1fr;
  }
  
  .insights-grid {
    grid-template-columns: 1fr;
  }
}
`;
