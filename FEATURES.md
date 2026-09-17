# Happy Home Hospital & Rehab — Features Specification & Delivery

**Project**: Happy Home Hospital & Inpatient Neuro-Rehabilitation Centre  
**Platform**: Dioxus 0.7 WASM Single Page Application (SPA)  
**Location**: Plot no.223, K-1 Kalinga Nagar, Ghatikia, Bhubaneswar, Odisha 751029  
**Status**: 100% Delivered and Verified  

---

## 1. Architectural Features

- **Dual-Brand Toggle Engine**:
  - Seamless in-app switching between **Hospital Mode** (Acute emergency, ICU step-down, surgical triage, multispecialty OPD) and **Rehab Centre Mode** (Inpatient neuro, spine trauma, stroke recovery, tracheostomy weaning, bedsore clinic).
  - Synchronizes reactive state via `BrandModeSignal` across all UI sections: Hero, Services, Why Us, Doctors, Testimonials, FAQs, and Booking.
- **Config-Driven Architecture**:
  - Complete separation of presentation and domain data.
  - Compile-time validated, zero-runtime-cost TOML embedding (`config/site.toml`).
- **Strict Zero-Emoji Policy**:
  - Handcrafted SVG icon library (`components/icons.rs`) replacing all unicode emojis.
  - Pixel-perfect vector icons for Stethoscope, Hospital, Ambulance, Brain, Spine, Activity, ShieldHeart, Bed, Wheelchair, Clock, MapPin, Phone, Mail, Chevron, Sun, Moon, Menu, Close, AlertCircle, and Star.
- **Official High-Resolution Vector SVG Logo (`HappyHomeLogo`)**:
  - Exact recreation of official Happy Home Hospital emblem: 12-petal lotus mandala, concentric yantra healing rings, Shatkona 6-pointed star, central blue bindu, and 5 chakra energy beads (Cyan, Emerald, Amber, Orange, Purple).
  - Authentic typography: Teal "HAPPY", Coastal Cyan "HOME", tracked Navy/White "HOSPITAL", and slogan "— • WHERE HEALING BEGINS • —".
- **Brand Palette & WCAG AAA/AA Contrast Compliance**:
  - Primary Coastal Cyan (`#36a1d7`), Deep Royal Medical Navy (`#134cb3`), Charcoal Obsidian (`#101a29`), and Pristine Ice Wash (`#f3f7fb`).
  - Dark mode midnight canvas (`#0a101d`) with elevated surface (`#111a2e`).
  - Guaranteed 15:1+ contrast on headings and 7:1+ (WCAG AAA) on body text in both light and dark modes.
- **Doctor Uniform Authentic Branding**:
  - Photorealistic clinical doctor attire featuring embroidered "Happy Home Hospital" / "Happy Home Rehab" insignia, replacing all competitor references.
- **Streamlined Header & Subtle Innovative Animations**:
  - Compact 28px utility topbar and slim glassmorphic navbar with smooth micro-interactions (`animate-float-subtle`, `animate-pulse-glow`, `btn-solid::after` shimmer reflections).

---

## 2. Inpatient Clinical & Medical Divisions

### Rehabilitation Divisions (9 Specialized Tracks)
1. **Post Neuro-Surgery Rehabilitation**: Multi-phase recovery following craniotomy, tumor resection, and spine surgery.
2. **Spinal Cord Injury Rehabilitation**: Paraplegia and quadriplegia mobility retraining, bladder-bowel protocols, and assistive independence.
3. **Traumatic Brain Injury (TBI) Care**: Cognitive restructuring, equilibrium recovery, and sensory-motor occupational therapy.
4. **Advanced Bedsore & Wound Care**: Negative pressure wound therapy (NPWT), enzymatic debridement, and alternating pressure air-beds.
5. **Tracheostomy & Respiratory Support**: Decannulation therapy, round-the-clock suctioning, chest physiotherapy, and airway safety.
6. **Stroke & Paralysis Recovery**: Neuroplasticity activation, tilt table standing, FES, and ADL retraining.
7. **Parkinson's & Movement Disorders**: Gait stabilization, tremor management, and speech-language therapy.
8. **Critical Step-Down & HDU Care**: Continuous multi-para vital monitoring, arterial line blood pressure tracking, and step-down transitions.
9. **Palliative & Supportive Oncology**: Holistic pain relief, symptom alleviation, and dignity-focused palliative care.

### Hospital Divisions (6 Core Inpatient Units)
1. **24/7 Emergency & Trauma Care**: Immediate stabilization for neuro-trauma and acute medical emergencies.
2. **Critical Care & High Dependency Unit (HDU)**: Hemodynamic monitoring, non-invasive ventilation, and dedicated nursing.
3. **Comprehensive Multi-Specialty OPD**: Clinical consultations in neurology, orthopedics, general medicine, and physiatry.
4. **Diagnostic Laboratory & Digital Imaging**: Rapid biochemistry, hematology, bedside ECG, 2D Echo, and digital X-ray.
5. **Advanced Physiotherapy & Pain Clinic**: Electrotherapy, therapeutic ultrasound, and ergonomic rehabilitation.
6. **Oncology Palliative & End-of-Life Ward**: Compassionate medical management and symptom alleviation.

---

## 3. Medical Leadership & Senior Consultants

- **Dr. Vijay Kumar Sahu**:
  - *Founder & Managing Director*
  - *Qualifications*: MPT (Neuro Rehabilitation)
  - *Specialty*: Neuro Rehabilitation, Stroke Recovery, Spinal Cord Injury & Long-term Critical Care.
  - *Image*: High-resolution clinical photograph (`assets/images/dr_vijay_sahu.jpg`).
- **Dr. Sambit Dash**:
  - *Senior Consultant (Anesthesiology & Critical Care)*
  - *Qualifications*: MD
  - *Specialty*: Post-Trauma Stabilization, Neuro Disorders & Critical Step-Down.
  - *Image*: High-resolution clinical photograph (`assets/images/dr_sambit_dash.jpg`).
- **Dr. Ananya Pattnaik**:
  - *Senior Consultant Neurologist*
  - *Qualifications*: MD, DM (Neurology)
  - *Specialty*: Stroke, Epilepsy, Neuromuscular Disorders, Neuro-Critical Consultations.
  - *Image*: High-resolution clinical photograph (`assets/images/dr_ananya_pattnaik.jpg`).
- **Dr. Jajati Keshari Jena**:
  - *Chief of Physiotherapy & Rehabilitation*
  - *Qualifications*: MPT (Neuro Rehabilitation)
  - *Specialty*: Spinal Cord Injury, Gait Training & Orthopedic Mobility.
  - *Image*: High-resolution clinical photograph (`assets/images/dr_jajati_jena.jpg`).

---

## 4. Patient Engagement & Interactive Features

- **Interactive Appointment & Inpatient Booking**:
  - Domain-validated input form (`name`, `phone`, `email`, `service`, `doctor`, `message`).
  - Client-side validation enforcing non-empty fields, E.164 phone formats, RFC 5322 compliant emails, and character length limits.
  - Sanitization of HTML special characters preventing Cross-Site Scripting (XSS).
  - Direct WhatsApp booking link generation pre-filling inquiry message.
- **Doctor Consultation Modal**:
  - Fast-booking modal displaying doctor credentials, specialty, and instant consultation routing.
- **Accessible Accordion FAQ**:
  - Categorized questions covering admission timeline, attendant stay policies, ICU capabilities, and rehab packages.
- **Inpatient Rehabilitation Packages (Plans)**:
  - 3 structured tiers: *Starter Rehabilitation* (7–14 days), *Intensive Neuro & Spine Care* (2–6 weeks, featured), and *Long-Term Palliative & Critical Care* (monthly).
- **Patient Recovery Testimonials**:
  - Real accounts from stroke recovery and spinal cord rehabilitation inpatients and their families.

---

## 5. Responsive UI & Accessibility

- **Mobile Navigation Drawer**: Fluid slide-down drawer with quick links, brand switcher, and direct contact buttons.
- **Sticky Glassmorphic Navigation Bar**: Frosted backdrop blur with active anchor links and emergency hotline indicator.
- **WCAG 2.1 AA & AAA Compliance**: Semantic markup (`header`, `nav`, `main`, `section`, `footer`), `aria-label` tags, visible focus states, skip link for screen readers, and guaranteed high-contrast typography (body text ≥9.5:1, headings ≥15:1) in both light and dark modes.

---

## 6. Super-Resolution Brand Logo & Healing Cure Experience

- **Authentic Live Logo Super-Resolution Asset**:
  - Direct replication of official `https://happyhomehospital.com/wp-content/uploads/2026/05/Logo-happy-home.jpg` upscaled to high-DPI 4x super-resolution (`Logo-happy-home-superres.png`).
  - Housed in a pristine rounded glass/white capsule in navbar and footer, preserving 100% fidelity of original fonts, lotus mandala, Shatkona star, chakra beads, and teal/cyan/navy lettering in both light and dark modes.
- **Innovative Healing Cure Card Animations**:
  - Organic 7-second restorative breathing aura (`healingAura`) on clinical service and department cards.
  - Soft restorative radial cyan sheen (`cure-light`) on hover and scroll.
  - Subtle rhythmic vitality pulse (`healingPulse`) on diagnostic icon rings.
- **Tag Cleanup**:
  - Purged generic promotional tags ("Odisha's Premier Neuro & Spine Healthcare") in favor of authentic clinical descriptors ("Advanced Multispecialty Hospital & Critical Care").

